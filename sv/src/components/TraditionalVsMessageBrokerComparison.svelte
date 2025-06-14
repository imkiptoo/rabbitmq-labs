<script>
  import { onMount } from 'svelte';
  
  let activeScenario = 'normal';
  let isPlaying = true;
  let animationSpeed = 1000;
  let activeMessages = [];
  let animationFrame;
  let failureMode = false;
  let highLoad = false;

  const scenarios = {
    normal: '✅ Everything Working',
    failure: '💥 One Service Breaks', 
    highLoad: '🚀 Heavy Traffic'
  };

  const traditionalPattern = {
    name: '🔗 Traditional: Direct Calls',
    description: 'Services call each other directly',
    nodes: [
      { id: 'frontend', type: 'service', x: 50, y: 120, label: 'Frontend' },
      { id: 'auth', type: 'service', x: 200, y: 80, label: 'Auth Service' },
      { id: 'user', type: 'service', x: 200, y: 160, label: 'User Service' },
      { id: 'order', type: 'service', x: 350, y: 120, label: 'Order Service' },
      { id: 'payment', type: 'service', x: 500, y: 80, label: 'Payment' },
      { id: 'inventory', type: 'service', x: 500, y: 160, label: 'Inventory' },
      { id: 'notification', type: 'service', x: 650, y: 120, label: 'Notification' }
    ],
    connections: [
      { from: 'frontend', to: 'auth' },
      { from: 'frontend', to: 'user' },
      { from: 'frontend', to: 'order' },
      { from: 'order', to: 'payment' },
      { from: 'order', to: 'inventory' },
      { from: 'order', to: 'notification' }
    ]
  };

  const messageBrokerPattern = {
    name: '🚀 Message Broker: Smart Queue',
    description: 'Services talk through a message broker',
    nodes: [
      { id: 'frontend', type: 'service', x: 50, y: 120, label: 'Frontend' },
      { id: 'auth', type: 'service', x: 200, y: 80, label: 'Auth Service' },
      { id: 'user', type: 'service', x: 200, y: 160, label: 'User Service' },
      { id: 'broker', type: 'broker', x: 350, y: 120, label: 'Message Broker' },
      { id: 'order', type: 'service', x: 500, y: 80, label: 'Order Service' },
      { id: 'payment', type: 'service', x: 500, y: 160, label: 'Payment' },
      { id: 'inventory', type: 'service', x: 650, y: 80, label: 'Inventory' },
      { id: 'notification', type: 'service', x: 650, y: 160, label: 'Notification' }
    ],
    connections: [
      { from: 'frontend', to: 'auth' },
      { from: 'frontend', to: 'user' },
      { from: 'frontend', to: 'broker' },
      { from: 'broker', to: 'order' },
      { from: 'broker', to: 'payment' },
      { from: 'broker', to: 'inventory' },
      { from: 'broker', to: 'notification' }
    ]
  };

  onMount(() => {
    startAnimationLoop();
    startScenarioLoop();
    return () => {
      if (animationFrame) {
        cancelAnimationFrame(animationFrame);
      }
    };
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

  function startScenarioLoop() {
    if (!isPlaying) return;
    
    setTimeout(() => {
      simulateRequest();
      startScenarioLoop();
    }, 3000 + Math.random() * 2000); // Slower for easier viewing
  }

  function simulateRequest() {
    if (!isPlaying) return;
    
    // Traditional pattern - direct connections
    const traditionalConnections = getTraditionalFlow();
    animatePattern('traditional', traditionalConnections);
    
    // Message broker pattern - via broker
    const brokerConnections = getBrokerFlow();
    animatePattern('broker', brokerConnections);
  }

  function getTraditionalFlow() {
    const baseFlow = [
      { from: 'frontend', to: 'auth' },
      { from: 'frontend', to: 'user' },
      { from: 'frontend', to: 'order' }
    ];

    if (activeScenario === 'failure') {
      // In failure mode, payment service is down - causes cascade
      return [
        ...baseFlow,
        { from: 'order', to: 'payment', failed: true },
        // Due to failure, subsequent calls fail
        { from: 'order', to: 'inventory', failed: true },
        { from: 'order', to: 'notification', failed: true }
      ];
    } else if (activeScenario === 'highLoad') {
      // In high load, everything slows down and some calls timeout
      return [
        ...baseFlow,
        { from: 'order', to: 'payment', slow: true },
        { from: 'order', to: 'inventory', slow: true },
        { from: 'order', to: 'notification', timeout: true }
      ];
    } else {
      return [
        ...baseFlow,
        { from: 'order', to: 'payment' },
        { from: 'order', to: 'inventory' },
        { from: 'order', to: 'notification' }
      ];
    }
  }

  function getBrokerFlow() {
    const baseFlow = [
      { from: 'frontend', to: 'auth' },
      { from: 'frontend', to: 'user' },
      { from: 'frontend', to: 'broker' }
    ];

    if (activeScenario === 'failure') {
      // Even if payment service is down, message is queued
      return [
        ...baseFlow,
        { from: 'broker', to: 'order' },
        { from: 'broker', to: 'inventory' },
        { from: 'broker', to: 'notification' },
        // Payment service failure doesn't block others
        { from: 'broker', to: 'payment', queued: true }
      ];
    } else if (activeScenario === 'highLoad') {
      // Broker handles load smoothly
      return [
        ...baseFlow,
        { from: 'broker', to: 'order' },
        { from: 'broker', to: 'payment' },
        { from: 'broker', to: 'inventory' },
        { from: 'broker', to: 'notification' }
      ];
    } else {
      return [
        ...baseFlow,
        { from: 'broker', to: 'order' },
        { from: 'broker', to: 'payment' },
        { from: 'broker', to: 'inventory' },
        { from: 'broker', to: 'notification' }
      ];
    }
  }

  function animatePattern(patternType, connections) {
    const messageId = `msg-${patternType}-${Date.now()}-${Math.random()}`;
    
    const newActiveMessage = {
      id: messageId,
      pattern: patternType,
      path: connections,
      currentStep: 0,
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
      const connection = message.path[message.currentStep];
      const delay = connection.slow ? animationSpeed * 2 : 
                   connection.timeout ? animationSpeed * 3 : 
                   connection.failed ? animationSpeed * 0.5 : animationSpeed;
      
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
      }, delay);
    }
  }

  function getMessagePosition(message) {
    if (message.currentStep >= message.path.length) return null;
    
    const connection = message.path[message.currentStep];
    const pattern = message.pattern === 'traditional' ? traditionalPattern : messageBrokerPattern;
    const fromNode = pattern.nodes.find(n => n.id === connection.from);
    const toNode = pattern.nodes.find(n => n.id === connection.to);
    
    if (!fromNode || !toNode) return null;
    
    const stepStartTime = message.startTime + (message.currentStep * animationSpeed);
    const elapsed = Date.now() - stepStartTime;
    const progress = Math.max(0, Math.min(elapsed / animationSpeed, 1));
    const easedProgress = easeInOutCubic(progress);
    
    const startX = fromNode.x + 50;
    const startY = fromNode.y;
    const endX = toNode.x - 50;
    const endY = toNode.y;
    
    const currentX = startX + (endX - startX) * easedProgress;
    const currentY = startY + (endY - startY) * easedProgress;
    
    return { 
      x: currentX, 
      y: currentY, 
      progress: easedProgress,
      connection: connection
    };
  }

  function easeInOutCubic(t) {
    return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
  }

  function getNodeColor(type, failed = false) {
    if (failed) return '#EF4444';
    
    switch (type) {
      case 'service': return '#3B82F6';
      case 'broker': return '#8B5CF6';
      default: return '#6B7280';
    }
  }

  function getConnectionPath(from, to, pattern) {
    const patternData = pattern === 'traditional' ? traditionalPattern : messageBrokerPattern;
    const fromNode = patternData.nodes.find(n => n.id === from);
    const toNode = patternData.nodes.find(n => n.id === to);
    
    if (!fromNode || !toNode) return '';
    
    const fromX = fromNode.x + 50;
    const fromY = fromNode.y;
    const toX = toNode.x - 50;
    const toY = toNode.y;
    
    return `M ${fromX} ${fromY} L ${toX} ${toY}`;
  }

  function changeScenario(scenario) {
    activeScenario = scenario;
    activeMessages = [];
  }

  function togglePlayback() {
    isPlaying = !isPlaying;
    if (isPlaying) {
      startScenarioLoop();
    }
  }

  function isNodeFailed(nodeId) {
    return activeScenario === 'failure' && nodeId === 'payment';
  }

  function getMessageColor(connection) {
    if (connection.failed) return '#EF4444';
    if (connection.timeout) return '#F59E0B';
    if (connection.slow) return '#F97316';
    if (connection.queued) return '#10B981';
    return '#3B82F6';
  }
</script>

<div class="space-y-6">
  <div class="bg-gradient-to-r from-red-50 to-blue-50 p-6 rounded-lg">
    <h2 class="text-2xl font-semibold text-gray-900 mb-3">Traditional vs Message Broker Architecture</h2>
    <p class="text-gray-700 mb-4">Compare how traditional microservices and message broker patterns handle challenges:</p>
    <div class="grid grid-cols-2 gap-4 text-sm">
      <div class="bg-red-100 p-3 rounded">
        <h4 class="font-semibold text-red-800">Traditional Challenges</h4>
        <ul class="text-red-700 mt-2 space-y-1">
          <li>• Service Coupling: Direct API dependencies</li>
          <li>• Cascading Failures: One failure affects all</li>
          <li>• Scalability Bottlenecks: Synchronous processing</li>
          <li>• Performance Degradation: Blocking operations</li>
        </ul>
      </div>
      <div class="bg-blue-100 p-3 rounded">
        <h4 class="font-semibold text-blue-800">Message Broker Benefits</h4>
        <ul class="text-blue-700 mt-2 space-y-1">
          <li>• Loose Coupling: Services communicate via broker</li>
          <li>• Fault Tolerance: Messages queued during failures</li>
          <li>• Better Scalability: Asynchronous processing</li>
          <li>• Improved Performance: Non-blocking operations</li>
        </ul>
      </div>
    </div>
  </div>
  
  <!-- Scenario Selection -->
  <div class="bg-gray-50 p-4 rounded-lg">
    <div class="text-center mb-4">
      <h3 class="text-lg font-semibold text-gray-800 mb-2">🎮 Try Different Scenarios</h3>
      <p class="text-sm text-gray-600">Click a scenario below to see how each architecture handles it</p>
    </div>
    
    <div class="flex items-center justify-center space-x-4 mb-4">
      {#each Object.entries(scenarios) as [id, name]}
        <button
          class="px-6 py-3 rounded-lg border-2 transition-all font-medium {
            activeScenario === id 
              ? 'border-blue-500 bg-blue-50 text-blue-900 scale-105 shadow-lg' 
              : 'border-gray-200 hover:border-gray-300 bg-white hover:scale-105'
          }"
          on:click={() => changeScenario(id)}
        >
          {name}
        </button>
      {/each}
    </div>
    
    <div class="flex items-center justify-center space-x-4">
      <button
        class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
        on:click={togglePlayback}
      >
        {isPlaying ? '⏸️ Pause' : '▶️ Play'} Auto Demo
      </button>
      
      <button
        class="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
        on:click={simulateRequest}
      >
        🚀 Send Test Request
      </button>
    </div>
  </div>

  <!-- Comparison Visualization -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Traditional Pattern -->
    <div class="bg-white border-2 border-red-200 rounded-lg p-6">
      <div class="mb-4">
        <h3 class="text-lg font-semibold text-red-800">{traditionalPattern.name}</h3>
        <p class="text-red-600 text-sm">{traditionalPattern.description}</p>
      </div>
      
      <div class="relative h-80 overflow-hidden">
        <svg width="100%" height="100%" class="absolute inset-0">
          <!-- Connection lines -->
          {#each traditionalPattern.connections as connection}
            {@const isActive = activeMessages.some(msg => {
              if (msg.pattern !== 'traditional' || msg.currentStep >= msg.path.length) return false;
              const currentConnection = msg.path[msg.currentStep];
              return currentConnection && currentConnection.from === connection.from && currentConnection.to === connection.to;
            })}
            <path
              d={getConnectionPath(connection.from, connection.to, 'traditional')}
              stroke={isActive ? "#EF4444" : "#E5E7EB"}
              stroke-width={isActive ? "3" : "2"}
              fill="none"
              stroke-dasharray={isActive ? "5,5" : "none"}
              marker-end="url(#arrowhead-traditional)"
            >
              {#if isActive}
                <animate
                  attributeName="stroke-dashoffset"
                  values="0;10"
                  dur="1s"
                  repeatCount="indefinite"
                />
              {/if}
            </path>
          {/each}
          
          <!-- Animated messages -->
          {#each activeMessages.filter(m => m.pattern === 'traditional') as message}
            {@const position = getMessagePosition(message)}
            {#if position}
              <circle
                cx={position.x}
                cy={position.y}
                r="4"
                fill={getMessageColor(position.connection)}
                stroke="white"
                stroke-width="2"
                class="message-dot"
              >
                <animate
                  attributeName="r"
                  values="4;6;4"
                  dur="0.8s"
                  repeatCount="indefinite"
                />
              </circle>
              
              <!-- Error indicator for failed messages -->
              {#if position.connection.failed}
                <text
                  x={position.x}
                  y={position.y - 15}
                  text-anchor="middle"
                  fill="#EF4444"
                  font-size="16"
                  font-weight="bold"
                >
                  💥
                </text>
              {:else if position.connection.slow}
                <text
                  x={position.x}
                  y={position.y - 15}
                  text-anchor="middle"
                  fill="#F59E0B"
                  font-size="14"
                  font-weight="bold"
                >
                  🐌
                </text>
              {/if}
            {/if}
          {/each}
          
          <!-- Definitions -->
          <defs>
            <marker id="arrowhead-traditional" markerWidth="10" markerHeight="7" 
              refX="5" refY="3.5" orient="auto">
              <polygon points="0 0, 5 3.5, 0 7" fill="#E5E7EB" />
            </marker>
          </defs>
        </svg>
        
        <!-- Nodes -->
        {#each traditionalPattern.nodes as node}
          {@const failed = isNodeFailed(node.id)}
          <div
            class="absolute flex items-center justify-center min-w-[100px] h-8 px-3 rounded-lg border-2 text-white text-xs font-medium transform -translate-x-1/2 -translate-y-1/2 {failed ? 'animate-pulse' : ''}"
            style="left: {node.x}px; top: {node.y}px; background-color: {getNodeColor(node.type, failed)}; border-color: {failed ? '#EF4444' : 'white'}"
          >
            {node.label}
            {#if failed}
              <span class="ml-1 text-white">⚠</span>
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Message Broker Pattern -->
    <div class="bg-white border-2 border-blue-200 rounded-lg p-6">
      <div class="mb-4">
        <h3 class="text-lg font-semibold text-blue-800">{messageBrokerPattern.name}</h3>
        <p class="text-blue-600 text-sm">{messageBrokerPattern.description}</p>
      </div>
      
      <div class="relative h-80 overflow-hidden">
        <svg width="100%" height="100%" class="absolute inset-0">
          <!-- Connection lines -->
          {#each messageBrokerPattern.connections as connection}
            {@const isActive = activeMessages.some(msg => {
              if (msg.pattern !== 'broker' || msg.currentStep >= msg.path.length) return false;
              const currentConnection = msg.path[msg.currentStep];
              return currentConnection && currentConnection.from === connection.from && currentConnection.to === connection.to;
            })}
            <path
              d={getConnectionPath(connection.from, connection.to, 'broker')}
              stroke={isActive ? "#3B82F6" : "#E5E7EB"}
              stroke-width={isActive ? "3" : "2"}
              fill="none"
              marker-end="url(#arrowhead-broker)"
            />
          {/each}
          
          <!-- Animated messages -->
          {#each activeMessages.filter(m => m.pattern === 'broker') as message}
            {@const position = getMessagePosition(message)}
            {#if position}
              <circle
                cx={position.x}
                cy={position.y}
                r="4"
                fill={getMessageColor(position.connection)}
                stroke="white"
                stroke-width="2"
                class="message-dot"
              >
                <animate
                  attributeName="r"
                  values="4;6;4"
                  dur="0.8s"
                  repeatCount="indefinite"
                />
              </circle>
              
              <!-- Queue indicator for queued messages -->
              {#if position.connection.queued}
                <text
                  x={position.x}
                  y={position.y - 15}
                  text-anchor="middle"
                  fill="#10B981"
                  font-size="16"
                  font-weight="bold"
                >
                  📦
                </text>
              {/if}
            {/if}
          {/each}
          
          <!-- Definitions -->
          <defs>
            <marker id="arrowhead-broker" markerWidth="10" markerHeight="7" 
              refX="5" refY="3.5" orient="auto">
              <polygon points="0 0, 5 3.5, 0 7" fill="#E5E7EB" />
            </marker>
          </defs>
        </svg>
        
        <!-- Nodes -->
        {#each messageBrokerPattern.nodes as node}
          <div
            class="absolute flex items-center justify-center min-w-[100px] h-8 px-3 rounded-lg border-2 border-white text-white text-xs font-medium transform -translate-x-1/2 -translate-y-1/2"
            style="left: {node.x}px; top: {node.y}px; background-color: {getNodeColor(node.type)}"
          >
            {node.label}
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Scenario Explanation -->
  <div class="bg-white border rounded-lg p-6">
    <h3 class="text-xl font-semibold mb-4 text-center">📖 What's Happening: {scenarios[activeScenario]}</h3>
    
    {#if activeScenario === 'normal'}
      <div class="bg-green-50 border border-green-200 rounded-lg p-4 mb-4">
        <h4 class="text-lg font-semibold text-green-800 mb-3">✅ Everything Working Normally</h4>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="bg-white p-3 rounded border-l-4 border-red-400">
            <h5 class="font-medium text-red-700 mb-2">🔗 Traditional Way</h5>
            <p class="text-sm text-gray-700">Each service calls the next service directly. Like a chain - each link depends on the next one working.</p>
          </div>
          <div class="bg-white p-3 rounded border-l-4 border-blue-400">
            <h5 class="font-medium text-blue-700 mb-2">🚀 Message Broker Way</h5>
            <p class="text-sm text-gray-700">Services send messages to a smart queue. The queue delivers messages when services are ready.</p>
          </div>
        </div>
      </div>
    {:else if activeScenario === 'failure'}
      <div class="bg-red-50 border border-red-200 rounded-lg p-4 mb-4">
        <h4 class="text-lg font-semibold text-red-800 mb-3">💥 Payment Service is Down!</h4>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="bg-white p-3 rounded border-l-4 border-red-500">
            <h5 class="font-medium text-red-700 mb-2">😰 Traditional Problem</h5>
            <p class="text-sm text-gray-700"><strong>Everything breaks!</strong> When payment fails, the whole order process stops. Users can't place orders at all.</p>
            <div class="mt-2 text-xs text-red-600">❌ Order → Payment (FAILS) → Everything stops</div>
          </div>
          <div class="bg-white p-3 rounded border-l-4 border-green-500">
            <h5 class="font-medium text-green-700 mb-2">😊 Message Broker Solution</h5>
            <p class="text-sm text-gray-700"><strong>Keep working!</strong> Orders go to the queue. When payment service comes back online, it processes all waiting orders.</p>
            <div class="mt-2 text-xs text-green-600">✅ Order → Queue → Payment (when ready)</div>
          </div>
        </div>
      </div>
    {:else if activeScenario === 'highLoad'}
      <div class="bg-orange-50 border border-orange-200 rounded-lg p-4 mb-4">
        <h4 class="text-lg font-semibold text-orange-800 mb-3">🚀 Lots of Users Right Now!</h4>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="bg-white p-3 rounded border-l-4 border-red-500">
            <h5 class="font-medium text-red-700 mb-2">😓 Traditional Struggle</h5>
            <p class="text-sm text-gray-700"><strong>Everything slows down!</strong> Each service waits for the previous one. Like a traffic jam - everyone gets stuck.</p>
            <div class="mt-2 text-xs text-red-600">🐌 Slow → Slower → Even Slower</div>
          </div>
          <div class="bg-white p-3 rounded border-l-4 border-green-500">
            <h5 class="font-medium text-green-700 mb-2">😎 Message Broker Handles It</h5>
            <p class="text-sm text-gray-700"><strong>Smooth sailing!</strong> Messages wait in line. Each service works at its own speed without blocking others.</p>
            <div class="mt-2 text-xs text-green-600">📦 Queue manages the traffic flow</div>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Legend -->
  <div class="bg-gray-50 p-4 rounded-lg">
    <h4 class="font-medium mb-3">Legend</h4>
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 text-sm">
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #3B82F6"></div>
        <span>Service</span>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #8B5CF6"></div>
        <span>Message Broker</span>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #EF4444"></div>
        <span>Failed/Error</span>
      </div>
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 rounded" style="background-color: #10B981"></div>
        <span>Queued Message</span>
      </div>
    </div>
  </div>
</div>

<style>
  .message-dot {
    filter: drop-shadow(0 0 3px rgba(0, 0, 0, 0.3));
  }
  
  .animate-pulse {
    animation: pulse 1s cubic-bezier(0.4, 0, 0.6, 1) infinite;
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