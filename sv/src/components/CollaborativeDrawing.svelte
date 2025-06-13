<script>
  import { onMount, onDestroy } from 'svelte';
  
  export let ws;
  export let connected;
  
  let canvas;
  let ctx;
  let isDrawing = false;
  let userId = '';
  let userColor = '#FF0000';
  let brushSize = 3;
  let lastX = 0;
  let lastY = 0;
  let cursors = new Map();
  let showCursors = true;
  
  const colors = [
    '#FF0000', '#00FF00', '#0000FF', '#FFFF00', '#FF00FF', '#00FFFF',
    '#FFA500', '#800080', '#008000', '#FFC0CB', '#A52A2A', '#000080'
  ];
  
  let websocketHandler;
  
  onMount(() => {
    // Generate unique user ID
    userId = 'user_' + Math.random().toString(36).substr(2, 9);
    
    if (canvas) {
      ctx = canvas.getContext('2d');
      setupCanvas();
    }
    
    if (typeof window !== 'undefined') {
      websocketHandler = handleWebSocketMessage;
      window.addEventListener('websocket-message', websocketHandler);
    }
  });
  
  onDestroy(() => {
    if (typeof window !== 'undefined' && websocketHandler) {
      window.removeEventListener('websocket-message', websocketHandler);
    }
  });
  
  function setupCanvas() {
    if (!canvas || !ctx) return;
    
    const rect = canvas.getBoundingClientRect();
    canvas.width = 800;
    canvas.height = 500;
    
    // Set up canvas for drawing
    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';
  }
  
  function handleWebSocketMessage(event) {
    const message = event.detail;
    
    if (message.demo_type === 'collaborative_drawing') {
      const data = message.data;
      
      switch (data.action) {
        case 'drawing_event':
          if (data.event.user_id !== userId) {
            handleRemoteDrawingEvent(data.event);
          }
          break;
        case 'cursor_move':
          if (data.cursor.user_id !== userId) {
            updateRemoteCursor(data.cursor);
          }
          break;
        case 'clear_canvas':
          clearCanvas();
          break;
      }
    }
  }
  
  function handleRemoteDrawingEvent(event) {
    if (!ctx) return;
    
    ctx.strokeStyle = event.color;
    ctx.lineWidth = event.brush_size;
    
    if (event.event_type === 'draw' && event.prev_x !== null && event.prev_y !== null) {
      ctx.beginPath();
      ctx.moveTo(event.prev_x, event.prev_y);
      ctx.lineTo(event.x, event.y);
      ctx.stroke();
    }
  }
  
  function updateRemoteCursor(cursor) {
    cursors.set(cursor.user_id, cursor);
    cursors = cursors; // trigger reactivity
    
    // Remove cursor after 3 seconds of inactivity
    setTimeout(() => {
      cursors.delete(cursor.user_id);
      cursors = cursors;
    }, 3000);
  }
  
  function startDrawing(e) {
    if (!connected || !ctx) return;
    
    isDrawing = true;
    const rect = canvas.getBoundingClientRect();
    lastX = e.clientX - rect.left;
    lastY = e.clientY - rect.top;
    
    sendDrawingEvent('start', lastX, lastY);
  }
  
  function draw(e) {
    if (!isDrawing || !connected || !ctx) return;
    
    const rect = canvas.getBoundingClientRect();
    const currentX = e.clientX - rect.left;
    const currentY = e.clientY - rect.top;
    
    // Draw locally for immediate feedback
    ctx.strokeStyle = userColor;
    ctx.lineWidth = brushSize;
    ctx.beginPath();
    ctx.moveTo(lastX, lastY);
    ctx.lineTo(currentX, currentY);
    ctx.stroke();
    
    // Send drawing event to other users
    sendDrawingEvent('draw', currentX, currentY, lastX, lastY);
    
    lastX = currentX;
    lastY = currentY;
  }
  
  function stopDrawing() {
    if (!isDrawing) return;
    isDrawing = false;
    sendDrawingEvent('end', lastX, lastY);
  }
  
  function handleMouseMove(e) {
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    // Send cursor position
    if (connected) {
      sendCursorMove(x, y);
    }
    
    if (isDrawing) {
      draw(e);
    }
  }
  
  async function sendDrawingEvent(eventType, x, y, prevX = null, prevY = null) {
    try {
      await fetch('http://localhost:3030/api/drawing/event', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          user_id: userId,
          event_type: eventType,
          x: x,
          y: y,
          prev_x: prevX,
          prev_y: prevY,
          color: userColor,
          brush_size: brushSize
        })
      });
    } catch (error) {
      console.error('Error sending drawing event:', error);
    }
  }
  
  async function sendCursorMove(x, y) {
    try {
      await fetch('http://localhost:3030/api/drawing/cursor', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          user_id: userId,
          x: x,
          y: y,
          color: userColor
        })
      });
    } catch (error) {
      console.error('Error sending cursor move:', error);
    }
  }
  
  async function clearCanvas() {
    if (!ctx) return;
    
    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }
  
  async function handleClearCanvas() {
    try {
      await fetch('http://localhost:3030/api/drawing/clear', {
        method: 'POST'
      });
    } catch (error) {
      console.error('Error clearing canvas:', error);
    }
  }
  
  // Touch events for mobile
  function handleTouchStart(e) {
    e.preventDefault();
    const touch = e.touches[0];
    const mouseEvent = new MouseEvent('mousedown', {
      clientX: touch.clientX,
      clientY: touch.clientY
    });
    startDrawing(mouseEvent);
  }
  
  function handleTouchMove(e) {
    e.preventDefault();
    const touch = e.touches[0];
    const mouseEvent = new MouseEvent('mousemove', {
      clientX: touch.clientX,
      clientY: touch.clientY
    });
    handleMouseMove(mouseEvent);
  }
  
  function handleTouchEnd(e) {
    e.preventDefault();
    stopDrawing();
  }
</script>

<div class="space-y-6">
  <div class="bg-blue-50 p-4 rounded-lg">
    <h2 class="text-xl font-semibold text-blue-900 mb-2">Collaborative Drawing Canvas</h2>
    <p class="text-blue-700">Draw together in real-time! All changes are instantly shared via RabbitMQ.</p>
  </div>
  
  <!-- Controls -->
  <div class="bg-white border rounded-lg p-4">
    <div class="flex flex-wrap items-center gap-4">
      <!-- Color Picker -->
      <div class="flex items-center space-x-2">
        <label class="text-sm font-medium text-gray-700">Color:</label>
        <div class="flex space-x-1">
          {#each colors as color}
            <button
              class="w-8 h-8 rounded-full border-2 transition-transform hover:scale-110 {
                userColor === color ? 'border-gray-800 scale-110' : 'border-gray-300'
              }"
              style="background-color: {color}"
              on:click={() => userColor = color}
            ></button>
          {/each}
        </div>
        <input
          type="color"
          bind:value={userColor}
          class="w-12 h-8 border border-gray-300 rounded"
        />
      </div>
      
      <!-- Brush Size -->
      <div class="flex items-center space-x-2">
        <label class="text-sm font-medium text-gray-700">Size:</label>
        <input
          type="range"
          min="1"
          max="20"
          bind:value={brushSize}
          class="w-24"
        />
        <span class="text-sm text-gray-600 w-8">{brushSize}px</span>
      </div>
      
      <!-- Clear Button -->
      <button
        class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors disabled:opacity-50"
        on:click={handleClearCanvas}
        disabled={!connected}
      >
        Clear Canvas
      </button>
      
      <!-- Cursor Toggle -->
      <label class="flex items-center space-x-2">
        <input
          type="checkbox"
          bind:checked={showCursors}
          class="rounded"
        />
        <span class="text-sm text-gray-700">Show Other Cursors</span>
      </label>
      
      <!-- Connection Status -->
      <div class="flex items-center space-x-2 ml-auto">
        <div class="w-3 h-3 rounded-full {connected ? 'bg-green-500' : 'bg-red-500'}"></div>
        <span class="text-sm text-gray-600">
          {connected ? 'Connected' : 'Disconnected'}
        </span>
      </div>
    </div>
  </div>
  
  <!-- Canvas Container -->
  <div class="bg-white border rounded-lg p-4">
    <div class="relative">
      <canvas
        bind:this={canvas}
        class="border border-gray-300 rounded cursor-crosshair touch-none"
        style="touch-action: none;"
        on:mousedown={startDrawing}
        on:mousemove={handleMouseMove}
        on:mouseup={stopDrawing}
        on:mouseleave={stopDrawing}
        on:touchstart={handleTouchStart}
        on:touchmove={handleTouchMove}
        on:touchend={handleTouchEnd}
      ></canvas>
      
      <!-- Remote Cursors -->
      {#if showCursors}
        {#each Array.from(cursors.values()) as cursor}
          <div
            class="absolute pointer-events-none transition-all duration-100"
            style="left: {cursor.x}px; top: {cursor.y}px; transform: translate(-50%, -50%);"
          >
            <div
              class="w-4 h-4 rounded-full border-2 border-white"
              style="background-color: {cursor.color};"
            ></div>
            <div class="absolute top-5 left-0 bg-black text-white text-xs px-2 py-1 rounded whitespace-nowrap">
              {cursor.user_id}
            </div>
          </div>
        {/each}
      {/if}
    </div>
    
    <div class="mt-4 text-sm text-gray-600">
      <strong>Your ID:</strong> {userId} | 
      <strong>Active Cursors:</strong> {cursors.size}
    </div>
  </div>
  
  <!-- Instructions -->
  <div class="bg-gray-50 p-4 rounded-lg">
    <h4 class="font-medium mb-2">How it works</h4>
    <ul class="text-sm text-gray-600 space-y-1">
      <li>• Click and drag to draw on the canvas</li>
      <li>• All drawing events are broadcast via RabbitMQ fanout exchange</li>
      <li>• See other users' cursors in real-time</li>
      <li>• Changes appear instantly across all connected sessions</li>
      <li>• Use different colors and brush sizes</li>
      <li>• Works on both desktop and mobile devices</li>
    </ul>
  </div>
</div>