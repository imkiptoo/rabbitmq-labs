<script>
  import { onMount } from 'svelte';
  
  export let ws;
  export let connected;
  
  let selectedDemo = 'logger';
  let messages = [];
  let isPlaying = true;
  let animationSpeed = 1000;
  let animationDuration = 2;
  
  const demos = {
    logger: {
      name: 'Simple Queue',
      description: 'Basic producer → queue → consumer pattern',
      nodes: [
        { id: 'producer', type: 'producer', x: 100, y: 200, label: 'Producer' },
        { id: 'queue', type: 'queue', x: 300, y: 200, label: 'messages' },
        { id: 'consumer', type: 'consumer', x: 500, y: 200, label: 'Consumer' }
      ],
      connections: [
        { from: 'producer', to: 'queue' },
        { from: 'queue', to: 'consumer' }
      ]
    },
    workers: {
      name: 'Work Queue',
      description: 'Producer → queue → multiple competing consumers',
      nodes: [
        { id: 'producer', type: 'producer', x: 100, y: 200, label: 'Producer' },
        { id: 'queue', type: 'queue', x: 300, y: 200, label: 'work_queue' },
        { id: 'worker1', type: 'consumer', x: 500, y: 150, label: 'Worker 1' },
        { id: 'worker2', type: 'consumer', x: 500, y: 200, label: 'Worker 2' },
        { id: 'worker3', type: 'consumer', x: 500, y: 250, label: 'Worker 3' }
      ],
      connections: [
        { from: 'producer', to: 'queue' },
        { from: 'queue', to: 'worker1' },
        { from: 'queue', to: 'worker2' },
        { from: 'queue', to: 'worker3' }
      ]
    },
    game: {
      name: 'Fanout Exchange',
      description: 'Producer → fanout exchange → multiple queues → consumers',
      nodes: [
        { id: 'producer', type: 'producer', x: 100, y: 200, label: 'Game Server' },
        { id: 'exchange', type: 'exchange', x: 250, y: 200, label: 'game_fanout' },
        { id: 'queue1', type: 'queue', x: 400, y: 150, label: 'player_1' },
        { id: 'queue2', type: 'queue', x: 400, y: 200, label: 'player_2' },
        { id: 'queue3', type: 'queue', x: 400, y: 250, label: 'player_3' },
        { id: 'player1', type: 'consumer', x: 550, y: 150, label: 'Player 1' },
        { id: 'player2', type: 'consumer', x: 550, y: 200, label: 'Player 2' },
        { id: 'player3', type: 'consumer', x: 550, y: 250, label: 'Player 3' }
      ],
      connections: [
        { from: 'producer', to: 'exchange' },
        { from: 'exchange', to: 'queue1' },
        { from: 'exchange', to: 'queue2' },
        { from: 'exchange', to: 'queue3' },
        { from: 'queue1', to: 'player1' },
        { from: 'queue2', to: 'player2' },
        { from: 'queue3', to: 'player3' }
      ]
    },
    rpc: {
      name: 'RPC Pattern',
      description: 'Request/Reply with correlation ID and reply queue',
      nodes: [
        { id: 'client', type: 'producer', x: 100, y: 200, label: 'RPC Client' },
        { id: 'request_queue', type: 'queue', x: 300, y: 200, label: 'rpc_queue' },
        { id: 'server', type: 'consumer', x: 500, y: 200, label: 'RPC Server' },
        { id: 'reply_queue', type: 'queue', x: 300, y: 280, label: 'reply_queue' }
      ],
      connections: [
        { from: 'client', to: 'request_queue' },
        { from: 'request_queue', to: 'server' },
        { from: 'server', to: 'reply_queue' },
        { from: 'reply_queue', to: 'client' }
      ]
    },
    drawing: {
      name: 'Drawing Board',
      description: 'Collaborative drawing with fanout exchange broadcasting',
      nodes: [
        { id: 'drawer', type: 'producer', x: 100, y: 200, label: 'Drawer' },
        { id: 'drawing_exchange', type: 'exchange', x: 250, y: 200, label: 'drawing_fanout' },
        { id: 'user1_queue', type: 'queue', x: 400, y: 150, label: 'user1_canvas' },
        { id: 'user2_queue', type: 'queue', x: 400, y: 200, label: 'user2_canvas' },
        { id: 'user3_queue', type: 'queue', x: 400, y: 250, label: 'user3_canvas' },
        { id: 'user1', type: 'consumer', x: 550, y: 150, label: 'User 1' },
        { id: 'user2', type: 'consumer', x: 550, y: 200, label: 'User 2' },
        { id: 'user3', type: 'consumer', x: 550, y: 250, label: 'User 3' }
      ],
      connections: [
        { from: 'drawer', to: 'drawing_exchange' },
        { from: 'drawing_exchange', to: 'user1_queue' },
        { from: 'drawing_exchange', to: 'user2_queue' },
        { from: 'drawing_exchange', to: 'user3_queue' },
        { from: 'user1_queue', to: 'user1' },
        { from: 'user2_queue', to: 'user2' },
        { from: 'user3_queue', to: 'user3' }
      ]
    }
  };
  
  let currentDemo = demos[selectedDemo];
  let activeMessages = [];
  let animationFrame;
  
  onMount(() => {
    if (typeof window !== 'undefined') {
      window.addEventListener('websocket-message', handleWebSocketMessage);
      startAnimationLoop();
      return () => {
        window.removeEventListener('websocket-message', handleWebSocketMessage);
        if (animationFrame) {
          cancelAnimationFrame(animationFrame);
        }
      };
    }
  });
  
  function startAnimationLoop() {
    function animate() {
      // Force reactivity update for message positions
      if (activeMessages.length > 0) {
        activeMessages = [...activeMessages];
      }
      animationFrame = requestAnimationFrame(animate);
    }
    animate();
  }
  
  function handleWebSocketMessage(event) {
    const message = event.detail;
    addMessage({
      id: Date.now(),
      demo: message.demo_type,
      data: message.data,
      timestamp: new Date().toLocaleTimeString()
    });
  }
  
  function addMessage(message) {
    messages = [message, ...messages].slice(0, 20);
    if (isPlaying) {
      animateMessage(message);
    }
  }
  
  function animateMessage(message) {
    const messageId = `msg-${Date.now()}-${Math.random()}`;
    const connections = currentDemo.connections;
    
    console.log('Starting animation for message:', messageId, 'with connections:', connections);
    
    const newActiveMessage = {
      id: messageId,
      path: connections,
      currentStep: 0,
      data: message.data,
      startTime: Date.now(),
      totalDuration: connections.length * animationSpeed
    };
    
    activeMessages = [...activeMessages, newActiveMessage];
    console.log('Active messages:', activeMessages.length);
    
    animateMessageFlow(messageId);
  }
  
  function animateMessageFlow(messageId) {
    const messageIndex = activeMessages.findIndex(m => m.id === messageId);
    if (messageIndex === -1) return;
    
    const message = activeMessages[messageIndex];
    
    if (message.currentStep < message.path.length) {
      setTimeout(() => {
        message.currentStep++;
        activeMessages = [...activeMessages];
        
        if (message.currentStep < message.path.length) {
          animateMessageFlow(messageId);
        } else {
          setTimeout(() => {
            activeMessages = activeMessages.filter(m => m.id !== messageId);
          }, 500);
        }
      }, animationSpeed);
    }
  }
  
  function getMessagePosition(message) {
    if (message.currentStep >= message.path.length) return null;
    
    const connection = message.path[message.currentStep];
    const fromNode = currentDemo.nodes.find(n => n.id === connection.from);
    const toNode = currentDemo.nodes.find(n => n.id === connection.to);
    
    if (!fromNode || !toNode) return null;
    
    // Calculate elapsed time for current step
    const stepStartTime = message.startTime + (message.currentStep * animationSpeed);
    const elapsed = Date.now() - stepStartTime;
    const progress = Math.max(0, Math.min(elapsed / animationSpeed, 1));
    
    // Use easing function for smoother movement
    const easedProgress = easeInOutCubic(progress);
    
    const startX = fromNode.x + 40;
    const startY = fromNode.y + 20;
    const endX = toNode.x;
    const endY = toNode.y + 20;
    
    const currentX = startX + (endX - startX) * easedProgress;
    const currentY = startY + (endY - startY) * easedProgress;
    
    return { x: currentX, y: currentY, progress: easedProgress };
  }
  
  function easeInOutCubic(t) {
    return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
  }
  
  function getActiveConnection(message) {
    if (message.currentStep >= message.path.length) return null;
    return message.path[message.currentStep];
  }
  
  function changeDemo(demoId) {
    selectedDemo = demoId;
    currentDemo = demos[demoId];
    activeMessages = [];
  }
  
  function togglePlayback() {
    isPlaying = !isPlaying;
  }
  
  function clearMessages() {
    messages = [];
    activeMessages = [];
  }
  
  function simulateMessage() {
    const demoMessages = {
      logger: { message: 'Hello from simulator!' },
      workers: { number: Math.floor(Math.random() * 100) },
      game: { player: 'SimPlayer', score: Math.floor(Math.random() * 10) },
      rpc: { request: 'status_check' },
      drawing: { user: 'SimUser', color: '#FF0000', points: [{x: 100, y: 100}, {x: 150, y: 150}] }
    };
    
    const newMessage = {
      id: Date.now(),
      demo: selectedDemo,
      data: demoMessages[selectedDemo],
      timestamp: new Date().toLocaleTimeString()
    };
    
    console.log('Simulating message:', newMessage);
    addMessage(newMessage);
  }
  
  function getNodeColor(type) {
    switch (type) {
      case 'producer': return '#3B82F6';
      case 'consumer': return '#10B981';
      case 'queue': return '#F59E0B';
      case 'exchange': return '#8B5CF6';
      default: return '#6B7280';
    }
  }
  
  function getConnectionPath(from, to) {
    const fromNode = currentDemo.nodes.find(n => n.id === from);
    const toNode = currentDemo.nodes.find(n => n.id === to);
    
    if (!fromNode || !toNode) return '';
    
    return `M ${fromNode.x + 40} ${fromNode.y + 20} L ${toNode.x} ${toNode.y + 20}`;
  }
</script>

<div class="space-y-6">
  <div class="bg-blue-50 p-4 rounded-lg">
    <h2 class="text-xl font-semibold text-blue-900 mb-2">Message Flow Simulator</h2>
    <p class="text-blue-700">Visualize how messages flow through RabbitMQ patterns in real-time</p>
  </div>
  
  <!-- Demo Selection -->
  <div class="grid grid-cols-5 gap-2">
    {#each Object.entries(demos) as [id, demo]}
      <button
        class="p-3 rounded-lg border-2 text-left transition-colors {
          selectedDemo === id 
            ? 'border-blue-500 bg-blue-50 text-blue-900' 
            : 'border-gray-200 hover:border-gray-300'
        }"
        on:click={() => changeDemo(id)}
      >
        <div class="font-medium">{demo.name}</div>
        <div class="text-xs text-gray-600 mt-1">{demo.description}</div>
      </button>
    {/each}
  </div>
  
  <!-- Controls -->
  <div class="flex items-center justify-between bg-gray-50 p-4 rounded-lg">
    <div class="flex items-center space-x-4">
      <button
        class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
        on:click={togglePlayback}
      >
        {isPlaying ? 'Pause' : 'Play'} Animation
      </button>
      
      <button
        class="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
        on:click={simulateMessage}
      >
        Simulate Message
      </button>
      
      <button
        class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
        on:click={clearMessages}
      >
        Clear
      </button>
      
      <div class="text-sm text-gray-600">
        Active: {activeMessages.length} | Playing: {isPlaying ? 'Yes' : 'No'}
      </div>
    </div>
    
    <div class="flex items-center space-x-2">
      <label class="text-sm text-gray-600">Speed:</label>
      <input
        type="range"
        min="500"
        max="3000"
        step="100"
        bind:value={animationSpeed}
        class="w-20"
      />
      <span class="text-sm text-gray-600">{animationSpeed}ms</span>
    </div>
  </div>
  
  <!-- Visualization -->
  <div class="bg-white border rounded-lg p-6">
    <div class="mb-4">
      <h3 class="text-lg font-semibold">{currentDemo.name}</h3>
      <p class="text-gray-600">{currentDemo.description}</p>
    </div>
    
    <div class="relative h-96 overflow-hidden">
      <svg width="100%" height="100%" class="absolute inset-0">
        <!-- Connection lines -->
        {#each currentDemo.connections as connection}
          {@const isActive = activeMessages.some(msg => {
            const activeConn = getActiveConnection(msg);
            return activeConn && activeConn.from === connection.from && activeConn.to === connection.to;
          })}
          <path
            d={getConnectionPath(connection.from, connection.to)}
            stroke={isActive ? "url(#activePathGradient)" : "#E5E7EB"}
            stroke-width={isActive ? "4" : "2"}
            fill="none"
            marker-end="url(#arrowhead)"
            class={isActive ? "active-path" : ""}
          >
            {#if isActive}
              <animate
                attributeName="stroke-opacity"
                values="0.5;1;0.5"
                dur="1.5s"
                repeatCount="indefinite"
              />
            {/if}
          </path>
        {/each}
        
        <!-- Animated messages -->
        {#each activeMessages as message}
          {@const position = getMessagePosition(message)}
          {#if position}
            <g>
              <!-- Message packet with glow effect -->
              <circle
                cx={position.x}
                cy={position.y}
                r="8"
                fill="url(#messageGradient)"
                stroke="#EF4444"
                stroke-width="2"
                class="message-packet"
              >
                <animate
                  attributeName="r"
                  values="6;10;6"
                  dur="0.8s"
                  repeatCount="indefinite"
                />
              </circle>
              
              <!-- Glow effect -->
              <circle
                cx={position.x}
                cy={position.y}
                r="12"
                fill="rgba(239, 68, 68, 0.3)"
                class="message-glow"
              >
                <animate
                  attributeName="opacity"
                  values="0.3;0.7;0.3"
                  dur="1s"
                  repeatCount="indefinite"
                />
              </circle>
              
              <!-- Data label -->
              <text
                x={position.x}
                y={position.y - 18}
                text-anchor="middle"
                fill="#374151"
                font-size="10"
                font-weight="bold"
                class="message-label"
              >
                {typeof message.data === 'object' ? 
                  Object.keys(message.data)[0] : 
                  String(message.data).substring(0, 8)}
              </text>
            </g>
          {/if}
        {/each}
        
        <!-- Definitions -->
        <defs>
          <marker id="arrowhead" markerWidth="10" markerHeight="7" 
            refX="10" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#E5E7EB" />
          </marker>
          
          <!-- Message gradient -->
          <radialGradient id="messageGradient" cx="50%" cy="50%" r="50%">
            <stop offset="0%" style="stop-color:#FCA5A5;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#EF4444;stop-opacity:1" />
          </radialGradient>
          
          <!-- Connection line gradient for active paths -->
          <linearGradient id="activePathGradient" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" style="stop-color:#3B82F6;stop-opacity:0.3" />
            <stop offset="50%" style="stop-color:#06B6D4;stop-opacity:0.6" />
            <stop offset="100%" style="stop-color:#10B981;stop-opacity:0.3" />
          </linearGradient>
        </defs>
      </svg>
      
      <!-- Nodes -->
      {#each currentDemo.nodes as node}
        <div
          class="absolute flex items-center justify-center w-20 h-10 rounded-lg border-2 border-white shadow-lg text-white text-xs font-medium transform -translate-x-1/2 -translate-y-1/2"
          style="left: {node.x}px; top: {node.y}px; background-color: {getNodeColor(node.type)}"
        >
          {node.label}
        </div>
      {/each}
    </div>
  </div>
  
  <!-- Message Log -->
  <div class="bg-white border rounded-lg">
    <div class="p-4 border-b bg-gray-50">
      <h3 class="font-semibold">Message Log</h3>
      <p class="text-sm text-gray-600">Real-time messages from all demos</p>
    </div>
    
    <div class="max-h-48 overflow-y-auto">
      {#if messages.length === 0}
        <div class="p-4 text-center text-gray-500">
          No messages yet. Try interacting with other demos or use "Simulate Message".
        </div>
      {:else}
        {#each messages as message}
          <div class="flex items-center justify-between p-3 border-b border-gray-100 hover:bg-gray-50">
            <div class="flex items-center space-x-3">
              <div class="w-2 h-2 rounded-full bg-blue-500"></div>
              <span class="font-medium text-blue-600">{message.demo}</span>
              <span class="text-sm text-gray-600">{JSON.stringify(message.data)}</span>
            </div>
            <span class="text-xs text-gray-400">{message.timestamp}</span>
          </div>
        {/each}
      {/if}
    </div>
  </div>
  
  <!-- Legend -->
  <div class="bg-gray-50 p-4 rounded-lg">
    <h4 class="font-medium mb-2">Legend</h4>
    <div class="grid grid-cols-4 gap-4 text-sm">
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #3B82F6"></div>
        <span>Producer</span>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #10B981"></div>
        <span>Consumer</span>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #F59E0B"></div>
        <span>Queue</span>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #8B5CF6"></div>
        <span>Exchange</span>
      </div>
    </div>
  </div>
</div>

<style>
  .message-packet {
    filter: drop-shadow(0 0 6px rgba(239, 68, 68, 0.6));
  }
  
  .message-glow {
    filter: blur(2px);
  }
  
  .message-label {
    text-shadow: 1px 1px 2px rgba(255, 255, 255, 0.8);
    pointer-events: none;
  }
  
  .active-path {
    filter: drop-shadow(0 0 4px rgba(59, 130, 246, 0.5));
  }
  
  svg {
    overflow: visible;
  }
  
  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
  
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: .5;
    }
  }
</style>