<script>
  import { onMount } from 'svelte';
  
  export let ws;
  export let connected;
  export let demoType = 'logger';

  let selectedDemo = demoType; // Lock to the specific demo type
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
        { id: 'queue', type: 'queue', x: 300, y: 200, label: 'Messages' },
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
        { id: 'queue', type: 'queue', x: 300, y: 200, label: 'Work Queue' },
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
        { id: 'exchange', type: 'exchange', x: 300, y: 200, label: 'Game Fanout' },
        { id: 'queue1', type: 'queue', x: 520, y: 150, label: 'Player 1 Queue' },
        { id: 'queue2', type: 'queue', x: 520, y: 200, label: 'Player 2 Queue' },
        { id: 'queue3', type: 'queue', x: 520, y: 250, label: 'Player 3 Queue' },
        { id: 'player1', type: 'consumer', x: 720, y: 150, label: 'Player 1' },
        { id: 'player2', type: 'consumer', x: 720, y: 200, label: 'Player 2' },
        { id: 'player3', type: 'consumer', x: 720, y: 250, label: 'Player 3' }
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
        { id: 'request_queue', type: 'queue', x: 300, y: 200, label: 'Request Queue' },
        { id: 'server', type: 'consumer', x: 500, y: 200, label: 'RPC Server' },
        { id: 'reply_queue', type: 'queue', x: 300, y: 280, label: 'Reply Queue' }
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
        { id: 'drawing_exchange', type: 'exchange', x: 280, y: 200, label: 'Drawing Fanout' },
        { id: 'user1_queue', type: 'queue', x: 480, y: 150, label: 'User 1 Canvas' },
        { id: 'user2_queue', type: 'queue', x: 480, y: 200, label: 'User 2 Canvas' },
        { id: 'user3_queue', type: 'queue', x: 480, y: 250, label: 'User 3 Canvas' },
        { id: 'user1', type: 'consumer', x: 680, y: 150, label: 'User 1' },
        { id: 'user2', type: 'consumer', x: 680, y: 200, label: 'User 2' },
        { id: 'user3', type: 'consumer', x: 680, y: 250, label: 'User 3' }
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
      if (activeMessages.length > 0) {
        activeMessages = [...activeMessages];
      }
      animationFrame = requestAnimationFrame(animate);
    }
    animate();
  }

  function handleWebSocketMessage(event) {
    const message = event.detail;
    if (message.demo_type === selectedDemo) {
      addMessage({
        id: Date.now(),
        demo: message.demo_type,
        data: message.data,
        timestamp: new Date().toLocaleTimeString()
      });
    }
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
    
    const newActiveMessage = {
      id: messageId,
      path: connections,
      currentStep: 0,
      data: message.data,
      startTime: Date.now(),
      totalDuration: connections.length * animationSpeed
    };
    
    activeMessages = [...activeMessages, newActiveMessage];
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
    
    const stepStartTime = message.startTime + (message.currentStep * animationSpeed);
    const elapsed = Date.now() - stepStartTime;
    const progress = Math.max(0, Math.min(elapsed / animationSpeed, 1));
    const easedProgress = easeInOutCubic(progress);
    
    const getNodeWidth = (node) => {
      if (node.type === 'exchange') return 160;
      return node.label.length > 10 ? 130 : 100;
    };
    
    const fromWidth = getNodeWidth(fromNode);
    const toWidth = getNodeWidth(toNode);
    
    let startX, startY, endX, endY;
    
    if (connection.from === 'server' && connection.to === 'reply_queue') {
      startX = fromNode.x;
      startY = fromNode.y + 20;
      endX = toNode.x + (toWidth / 2);
      endY = toNode.y;
    } else if (connection.from === 'reply_queue' && connection.to === 'client') {
      startX = fromNode.x - (fromWidth / 2);
      startY = fromNode.y;
      endX = toNode.x;
      endY = toNode.y + 20;
    } else {
      startX = fromNode.x + (fromWidth / 2);
      startY = fromNode.y;
      endX = toNode.x - (toWidth / 2);
      endY = toNode.y;
    }
    
    const currentX = startX + (endX - startX) * easedProgress;
    const currentY = startY + (endY - startY) * easedProgress;
    
    return { x: currentX, y: currentY, progress: easedProgress };
  }

  function easeInOutCubic(t) {
    return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
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
    
    const getNodeWidth = (node) => {
      if (node.type === 'exchange') return 160;
      return node.label.length > 10 ? 130 : 100;
    };
    
    const fromWidth = getNodeWidth(fromNode);
    const toWidth = getNodeWidth(toNode);
    
    let fromX, fromY, toX, toY;
    
    if (from === 'server' && to === 'reply_queue') {
      fromX = fromNode.x;
      fromY = fromNode.y + 20;
      toX = toNode.x + (toWidth / 2);
      toY = toNode.y;
    } else if (from === 'reply_queue' && to === 'client') {
      fromX = fromNode.x - (fromWidth / 2);
      fromY = fromNode.y;
      toX = toNode.x;
      toY = toNode.y + 20;
    } else {
      fromX = fromNode.x + (fromWidth / 2);
      fromY = fromNode.y;
      toX = toNode.x - (toWidth / 2);
      toY = toNode.y;
    }
    
    return `M ${fromX} ${fromY} L ${toX} ${toY}`;
  }
</script>

<div class="space-y-6">
  <div class="bg-blue-50 p-4 rounded-lg">
    <h2 class="text-xl font-semibold text-blue-900 mb-2">Flow Simulator - {currentDemo.name}</h2>
    <p class="text-blue-700">{currentDemo.description}</p>
  </div>
  
  <!-- Controls -->
  <div class="flex items-center justify-between bg-neutral-50 p-4 rounded-lg">
    <div class="flex items-center space-x-4">
      <button
        class="px-3 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
        on:click={togglePlayback}
      >
        {isPlaying ? 'Pause' : 'Play'} Animation
      </button>
      
      <button
        class="px-3 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
        on:click={simulateMessage}
      >
        Simulate Message
      </button>
      
      <button
        class="px-3 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
        on:click={clearMessages}
      >
        Clear
      </button>
      
      <div class="text-sm text-neutral-600">
        Active: {activeMessages.length} | Playing: {isPlaying ? 'Yes' : 'No'} | 
        Highlighted: {currentDemo.connections.filter(conn => 
          activeMessages.some(msg => {
            if (msg.currentStep >= msg.path.length) return false;
            const currentConnection = msg.path[msg.currentStep];
            return currentConnection && currentConnection.from === conn.from && currentConnection.to === conn.to;
          })
        ).length}
      </div>
    </div>
    
    <div class="flex items-center space-x-2">
      <label class="text-sm text-neutral-600">Speed:</label>
      <input
        type="range"
        min="500"
        max="3000"
        step="100"
        bind:value={animationSpeed}
        class="w-20"
      />
      <span class="text-sm text-neutral-600">{animationSpeed}ms</span>
    </div>
  </div>
  
  <!-- Visualization -->
  <div class="bg-white border rounded-lg p-6">
    <div class="mb-4">
      <h3 class="text-lg font-semibold">{currentDemo.name}</h3>
      <p class="text-neutral-600">{currentDemo.description}</p>
    </div>
    
    <div class="relative h-96 overflow-hidden">
      <svg width="100%" height="100%" class="absolute inset-0">
        <!-- Connection lines -->
        {#each currentDemo.connections as connection}
          {@const isActive = activeMessages.some(msg => {
            if (msg.currentStep >= msg.path.length) return false;
            const currentConnection = msg.path[msg.currentStep];
            return currentConnection && currentConnection.from === connection.from && currentConnection.to === connection.to;
          })}
          <!-- Base line (always visible) -->
          <path
            d={getConnectionPath(connection.from, connection.to)}
            stroke={isActive ? "#D1D5DB" : "#d0d0d0"}
            stroke-width="2"
            fill="none"
            marker-end={isActive ? "none" : "url(#arrowhead)"}
          />
          <!-- Active highlight effects -->
          {#if isActive}
            <!-- Outer glow -->
            <path
              d={getConnectionPath(connection.from, connection.to)}
              stroke="#FFA500"
              stroke-width="6"
              fill="none"
              stroke-linecap="round"
              opacity="0.3"
            >
              <animate
                attributeName="opacity"
                values="0.1;0.4;0.1"
                dur="1s"
                repeatCount="indefinite"
              />
            </path>
            <!-- Middle glow -->
            <path
              d={getConnectionPath(connection.from, connection.to)}
              stroke="#FF6B35"
              stroke-width="4"
              fill="none"
              stroke-linecap="round"
              opacity="0.6"
            >
              <animate
                attributeName="opacity"
                values="0.4;0.8;0.4"
                dur="1.2s"
                repeatCount="indefinite"
              />
            </path>
            <!-- Core highlight -->
            <path
              d={getConnectionPath(connection.from, connection.to)}
              stroke="#FF4444"
              stroke-width="2"
              fill="none"
              class="active-path-core"
              stroke-linecap="round"
            >
              <animate
                attributeName="stroke"
                values="#FF4444;#FF6B35;#FFA500;#FF6B35;#FF4444"
                dur="2s"
                repeatCount="indefinite"
              />
            </path>
            <!-- Active arrowhead on top using marker -->
            <path
              d={getConnectionPath(connection.from, connection.to)}
              stroke="transparent"
              stroke-width="0"
              fill="none"
              marker-end="url(#arrowhead-active-animated)"
            />
          {/if}
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
                r="2"
                fill="url(#messageGradient)"
                stroke="#FF4444"
                stroke-width="1"
                class="message-packet"
              >
                <animate
                  attributeName="r"
                  values="4;6;4"
                  dur="0.6s"
                  repeatCount="indefinite"
                />
                <animate
                  attributeName="stroke"
                  values="#FF4444;#FF6B35;#FFA500;#FF6B35;#FF4444"
                  dur="1.5s"
                  repeatCount="indefinite"
                />
              </circle>
              
              <!-- Glow effect -->
              <circle
                cx={position.x}
                cy={position.y}
                r="4"
                fill="rgba(255, 68, 68, 0.4)"
                class="message-glow"
              >
                <animate
                  attributeName="opacity"
                  values="0.2;0.8;0.2"
                  dur="0.8s"
                  repeatCount="indefinite"
                />
                <animate
                  attributeName="r"
                  values="7;9;7"
                  dur="1.2s"
                  repeatCount="indefinite"
                />
              </circle>
            </g>
          {/if}
        {/each}
        
        <!-- Definitions -->
        <defs>
          <marker id="arrowhead" markerWidth="10" markerHeight="7" 
            refX="5" refY="3.5" orient="auto">
            <polygon points="0 0, 5 3.5, 0 7" fill="#d0d0d0" />
          </marker>
          
          <marker id="arrowhead-active-animated" markerWidth="10" markerHeight="7" 
            refX="5" refY="3.5" orient="auto">
            <polygon points="0 0, 5 3.5, 0 7" fill="#FF4444">
              <animate
                attributeName="fill"
                values="#FF4444;#FF6B35;#FFA500;#FF6B35;#FF4444"
                dur="2s"
                repeatCount="indefinite"
              />
            </polygon>
          </marker>
          
          <!-- Message gradient -->
          <radialGradient id="messageGradient" cx="50%" cy="50%" r="50%">
            <stop offset="0%" style="stop-color:#FCA5A5;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#EF4444;stop-opacity:1" />
          </radialGradient>
        </defs>
      </svg>
      
      <!-- Nodes -->
      {#each currentDemo.nodes as node}
        {@const isLongLabel = node.label.length > 10}
        {@const isExchange = node.type === 'exchange'}
        {@const nodeWidth = isExchange ? 'min-w-[160px]' : (isLongLabel ? 'min-w-[130px]' : 'min-w-[100px]')}
        <div
          class="absolute flex items-center justify-center {nodeWidth} h-10 px-3 rounded-lg border-2 border-white shadow text-white text-sm font-medium transform -translate-x-1/2 -translate-y-1/2 whitespace-nowrap"
          style="left: {node.x}px; top: {node.y}px; background-color: {getNodeColor(node.type)}"
        >
          {node.label}
        </div>
      {/each}
    </div>
  </div>
  
  <!-- Message Log -->
  <div class="bg-white border rounded-lg">
    <div class="p-4 border-b bg-neutral-50">
      <h3 class="font-semibold">Message Log</h3>
      <p class="text-sm text-neutral-600">Real-time messages for {currentDemo.name}</p>
    </div>
    
    <div class="max-h-48 overflow-y-auto">
      {#if messages.length === 0}
        <div class="p-4 text-center text-neutral-500">
          No messages yet. Try using the demo above or click "Simulate Message".
        </div>
      {:else}
        {#each messages as message}
          <div class="flex items-center justify-between p-3 border-b border-neutral-100 hover:bg-neutral-50">
            <div class="flex items-center space-x-3">
              <div class="w-2 h-2 rounded-full bg-blue-500"></div>
              <span class="font-medium text-blue-600">{message.demo}</span>
              <span class="text-sm text-neutral-600">{JSON.stringify(message.data)}</span>
            </div>
            <span class="text-xs text-neutral-400">{message.timestamp}</span>
          </div>
        {/each}
      {/if}
    </div>
  </div>
  
  <!-- Legend -->
  <div class="bg-neutral-50 p-4 rounded-lg">
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
    filter: drop-shadow(0 0 5px rgba(255, 68, 68, 1)) 
            drop-shadow(0 0 10px rgba(255, 107, 53, 0.8)) 
            drop-shadow(0 0 15px rgba(255, 165, 0, 0.6));
  }
  
  .message-glow {
    filter: blur(1px);
  }
  
  .active-path-core {
    filter: drop-shadow(0 0 4px rgba(255, 68, 68, 1)) 
            drop-shadow(0 0 8px rgba(255, 107, 53, 0.8)) 
            drop-shadow(0 0 12px rgba(255, 165, 0, 0.6));
  }
  
  svg {
    overflow: visible;
  }
</style>