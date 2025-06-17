<script>
  import { onMount } from 'svelte';
  
  export let ws;
  export let connected;
  export let demoType = 'logger';

  let selectedDemo = demoType;
  let selectedSimulationType = 'message_patterns';
  let messages = [];
  let isPlaying = true;
  let animationSpeed = 1000;
  let animationDuration = 2;
  let activeScenario = 'normal';
  let failureMode = false;
  let highLoad = false;

  const simulationTypes = {
    message_patterns: 'Message Flow Patterns',
    architecture_comparison: 'Traditional vs Message Broker',
    rpc_pattern: 'Request/Reply (RPC)',
    work_queue: 'Work Queue Pattern'
  };

  const messagePatternDemos = {
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

  const architectureComparison = {
    traditional: {
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
    },
    broker: {
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
    }
  };

  const rpcDemo = {
    name: 'RPC Status Check',
    description: 'Client sends request, server responds',
    nodes: [
      { id: 'client', type: 'producer', x: 100, y: 200, label: 'Client' },
      { id: 'request_queue', type: 'queue', x: 300, y: 200, label: 'Request Queue' },
      { id: 'server', type: 'consumer', x: 500, y: 200, label: 'Server' },
      { id: 'reply_queue', type: 'queue', x: 300, y: 280, label: 'Reply Queue' }
    ],
    connections: [
      { from: 'client', to: 'request_queue' },
      { from: 'request_queue', to: 'server' },
      { from: 'server', to: 'reply_queue' },
      { from: 'reply_queue', to: 'client' }
    ]
  };

  const workQueueDemo = {
    name: 'Number Doubling Workers',
    description: 'Multiple workers compete for tasks from shared queue',
    nodes: [
      { id: 'producer', type: 'producer', x: 100, y: 200, label: 'Task Producer' },
      { id: 'work_queue', type: 'queue', x: 300, y: 200, label: 'Work Queue' },
      { id: 'worker1', type: 'consumer', x: 500, y: 150, label: 'Worker 1' },
      { id: 'worker2', type: 'consumer', x: 500, y: 200, label: 'Worker 2' },
      { id: 'worker3', type: 'consumer', x: 500, y: 250, label: 'Worker 3' }
    ],
    connections: [
      { from: 'producer', to: 'work_queue' },
      { from: 'work_queue', to: 'worker1' },
      { from: 'work_queue', to: 'worker2' },
      { from: 'work_queue', to: 'worker3' }
    ]
  };

  const scenarios = {
    normal: '✅ Everything Working',
    failure: '💥 One Service Breaks', 
    highLoad: '🚀 Heavy Traffic'
  };

  let currentDemo = getCurrentDemo();

  function getCurrentDemo() {
    switch (selectedSimulationType) {
      case 'message_patterns':
        return messagePatternDemos[selectedDemo];
      case 'architecture_comparison':
        return selectedDemo === 'traditional' ? architectureComparison.traditional : architectureComparison.broker;
      case 'rpc_pattern':
        return rpcDemo;
      case 'work_queue':
        return workQueueDemo;
      default:
        return messagePatternDemos[selectedDemo];
    }
  }
  let activeMessages = [];
  let animationFrame;

  onMount(() => {
    if (typeof window !== 'undefined') {
      window.addEventListener('websocket-message', handleWebSocketMessage);
      startAnimationLoop();
      
      // Auto-demo for architecture comparison
      if (selectedSimulationType === 'architecture_comparison') {
        startArchitectureDemo();
      }
      
      return () => {
        window.removeEventListener('websocket-message', handleWebSocketMessage);
        if (animationFrame) {
          cancelAnimationFrame(animationFrame);
        }
      };
    }
  });

  function startArchitectureDemo() {
    if (selectedSimulationType !== 'architecture_comparison' || !isPlaying) return;
    
    setTimeout(() => {
      simulateMessage();
      startArchitectureDemo();
    }, 4000 + Math.random() * 2000);
  }

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
    const messageType = selectedSimulationType === 'message_patterns' ? selectedDemo : 
                       selectedSimulationType === 'rpc_pattern' ? 'rpc' :
                       selectedSimulationType === 'work_queue' ? 'workers' : 
                       selectedDemo;
    
    if (message.demo_type === messageType) {
      addMessage({
        id: Date.now(),
        demo: message.demo_type,
        data: message.data,
        timestamp: new Date().toLocaleTimeString(),
        simulationType: selectedSimulationType
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
    if (selectedSimulationType === 'architecture_comparison') {
      animateArchitectureComparison();
      return;
    }
    
    const messageId = `msg-${Date.now()}-${Math.random()}`;
    const connections = currentDemo.connections;
    
    const newActiveMessage = {
      id: messageId,
      path: connections,
      currentStep: 0,
      data: message.data,
      startTime: Date.now(),
      totalDuration: connections.length * animationSpeed,
      simulationType: selectedSimulationType
    };
    
    activeMessages = [...activeMessages, newActiveMessage];
    animateMessageFlow(messageId);
  }

  function animateArchitectureComparison() {
    const traditionalConnections = getTraditionalFlow();
    const brokerConnections = getBrokerFlow();
    
    animatePattern('traditional', traditionalConnections);
    animatePattern('broker', brokerConnections);
  }

  function getTraditionalFlow() {
    const baseFlow = [
      { from: 'frontend', to: 'auth' },
      { from: 'frontend', to: 'user' },
      { from: 'frontend', to: 'order' }
    ];

    if (activeScenario === 'failure') {
      return [
        ...baseFlow,
        { from: 'order', to: 'payment', failed: true },
        { from: 'order', to: 'inventory', failed: true },
        { from: 'order', to: 'notification', failed: true }
      ];
    } else if (activeScenario === 'highLoad') {
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
      return [
        ...baseFlow,
        { from: 'broker', to: 'order' },
        { from: 'broker', to: 'inventory' },
        { from: 'broker', to: 'notification' },
        { from: 'broker', to: 'payment', queued: true }
      ];
    } else if (activeScenario === 'highLoad') {
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
      totalDuration: connections.length * animationSpeed,
      simulationType: 'architecture_comparison'
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
      const delay = connection?.slow ? animationSpeed * 2 : 
                   connection?.timeout ? animationSpeed * 3 : 
                   connection?.failed ? animationSpeed * 0.5 : animationSpeed;
      
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
    let pattern = currentDemo;
    
    if (message.simulationType === 'architecture_comparison') {
      pattern = message.pattern === 'traditional' ? architectureComparison.traditional : architectureComparison.broker;
    }
    
    const fromNode = pattern.nodes.find(n => n.id === connection.from);
    const toNode = pattern.nodes.find(n => n.id === connection.to);
    
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
    
    return { 
      x: currentX, 
      y: currentY, 
      progress: easedProgress, 
      connection: connection,
      pattern: message.pattern
    };
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
      drawing: { user: 'SimUser', color: '#FF0000', points: [{x: 100, y: 100}, {x: 150, y: 150}] },
      architecture_comparison: { request: 'user_order', service: 'order_processing' }
    };
    
    const messageKey = selectedSimulationType === 'message_patterns' ? selectedDemo :
                       selectedSimulationType === 'architecture_comparison' ? 'architecture_comparison' :
                       selectedSimulationType === 'rpc_pattern' ? 'rpc' :
                       selectedSimulationType === 'work_queue' ? 'workers' : selectedDemo;
    
    const newMessage = {
      id: Date.now(),
      demo: messageKey,
      data: demoMessages[messageKey],
      timestamp: new Date().toLocaleTimeString(),
      simulationType: selectedSimulationType
    };
    
    addMessage(newMessage);
  }

  function changeSimulationType(type) {
    selectedSimulationType = type;
    activeMessages = [];
    messages = [];
    
    if (type === 'message_patterns') {
      selectedDemo = demoType;
    } else if (type === 'architecture_comparison') {
      selectedDemo = 'traditional';
      if (isPlaying) {
        setTimeout(() => startArchitectureDemo(), 1000);
      }
    }
    
    currentDemo = getCurrentDemo();
  }

  function changeDemo(demoId) {
    selectedDemo = demoId;
    currentDemo = getCurrentDemo();
    activeMessages = [];
  }

  function changeScenario(scenario) {
    activeScenario = scenario;
    activeMessages = [];
  }

  function getNodeColor(type, failed = false) {
    if (failed) return '#EF4444';
    
    switch (type) {
      case 'producer': return '#3B82F6';
      case 'consumer': return '#10B981';
      case 'queue': return '#F59E0B';
      case 'exchange': return '#8B5CF6';
      case 'service': return '#3B82F6';
      case 'broker': return '#8B5CF6';
      default: return '#6B7280';
    }
  }

  function getMessageColor(connection) {
    if (connection?.failed) return '#EF4444';
    if (connection?.timeout) return '#F59E0B';
    if (connection?.slow) return '#F97316';
    if (connection?.queued) return '#10B981';
    return '#3B82F6';
  }

  function isNodeFailed(nodeId) {
    return activeScenario === 'failure' && nodeId === 'payment';
  }

  function getConnectionPath(from, to, pattern = null) {
    let patternData = pattern ? (pattern === 'traditional' ? architectureComparison.traditional : 
                                pattern === 'broker' ? architectureComparison.broker : currentDemo) : currentDemo;
    
    const fromNode = patternData.nodes.find(n => n.id === from);
    const toNode = patternData.nodes.find(n => n.id === to);
    
    if (!fromNode || !toNode) return '';
    
    const getNodeWidth = (node) => {
      if (node.type === 'exchange' || node.type === 'broker') return 160;
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

<div class="space-y-3">
  <!-- Simulation Type Tabs -->
  <div class="bg-white border rounded-lg">
    <div class="border-b">
      <div class="flex space-x-1 p-1">
        {#each Object.entries(simulationTypes) as [id, name]}
          <button
            class="px-4 py-2 rounded-md text-sm font-medium transition-colors {
              selectedSimulationType === id 
                ? 'bg-blue-500 text-white' 
                : 'text-neutral-600 hover:text-neutral-900 hover:bg-neutral-100'
            }"
            on:click={() => changeSimulationType(id)}
          >
            {name}
          </button>
        {/each}
      </div>
    </div>
    
    <div class="p-4">
      <h2 class="text-xl font-semibold text-blue-900 mb-2">
        {simulationTypes[selectedSimulationType]} - {currentDemo.name}
      </h2>
      <p class="text-blue-700">{currentDemo.description}</p>
    </div>
  </div>
  
  <!-- Sub-demo Selection (for message patterns and architecture comparison) -->
  {#if selectedSimulationType === 'message_patterns'}
    <div class="grid grid-cols-5 gap-2">
      {#each Object.entries(messagePatternDemos) as [id, demo]}
        <button
          class="p-3 rounded-lg border-2 text-left transition-colors {
            selectedDemo === id 
              ? 'border-blue-500 bg-blue-50 text-blue-900' 
              : 'border-neutral-200 hover:border-neutral-300'
          }"
          on:click={() => changeDemo(id)}
        >
          <div class="font-medium">{demo.name}</div>
          <div class="text-xs text-neutral-600 mt-1">{demo.description}</div>
        </button>
      {/each}
    </div>
  {:else if selectedSimulationType === 'architecture_comparison'}
    <div class="bg-neutral-50 p-4 rounded-lg">
      <div class="text-center mb-4">
        <h3 class="text-lg font-semibold text-neutral-800 mb-2">🎮 Try Different Scenarios</h3>
        <p class="text-sm text-neutral-600">Click a scenario below to see how each architecture handles it</p>
      </div>
      
      <div class="flex items-center justify-center space-x-3 mb-4">
        {#each Object.entries(scenarios) as [id, name]}
          <button
            class="px-6 py-3 rounded-lg border-2 transition-all font-medium {
              activeScenario === id 
                ? 'border-blue-500 bg-blue-50 text-blue-900 scale-105 shadow-lg' 
                : 'border-neutral-200 hover:border-neutral-300 bg-white hover:scale-105'
            }"
            on:click={() => changeScenario(id)}
          >
            {name}
          </button>
        {/each}
      </div>
      
      <div class="flex items-center justify-center space-x-3">
        <div class="grid grid-cols-2 gap-2">
          <button
            class="px-4 py-2 rounded-lg border-2 transition-colors {
              selectedDemo === 'traditional' 
                ? 'border-red-500 bg-red-50 text-red-900' 
                : 'border-neutral-200 hover:border-neutral-300'
            }"
            on:click={() => changeDemo('traditional')}
          >
            🔗 Traditional
          </button>
          <button
            class="px-4 py-2 rounded-lg border-2 transition-colors {
              selectedDemo === 'broker' 
                ? 'border-blue-500 bg-blue-50 text-blue-900' 
                : 'border-neutral-200 hover:border-neutral-300'
            }"
            on:click={() => changeDemo('broker')}
          >
            🚀 Message Broker
          </button>
        </div>
      </div>
    </div>
  {/if}
  
  <!-- Controls -->
  <div class="flex items-center justify-between bg-neutral-50 p-4 rounded-lg">
    <div class="flex items-center space-x-3">
      <button
        class="px-3 py-1 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
        on:click={togglePlayback}
      >
        {isPlaying ? 'Pause' : 'Play'} Animation
      </button>
      
      <button
        class="px-3 py-1 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
        on:click={simulateMessage}
      >
        Simulate Message
      </button>
      
      <button
        class="px-3 py-1 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
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
    
    <div class="relative px-3 py-1 overflow-hidden">
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
            {@const messageColor = getMessageColor(position.connection)}
            <g>
              <!-- Different message styles for different simulation types -->
              {#if selectedSimulationType === 'architecture_comparison'}
                <circle
                  cx={position.x}
                  cy={position.y}
                  r="4"
                  fill={messageColor}
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
                
                <!-- Error/Status indicators -->
                {#if position.connection?.failed}
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
                {:else if position.connection?.slow}
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
                {:else if position.connection?.queued}
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
              {:else}
                <!-- Default message packet for other simulations -->
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
              {/if}
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
        {@const isExchange = node.type === 'exchange' || node.type === 'broker'}
        {@const nodeWidth = isExchange ? 'min-w-[160px]' : (isLongLabel ? 'min-w-[130px]' : 'min-w-[100px]')}
        {@const failed = selectedSimulationType === 'architecture_comparison' && isNodeFailed(node.id)}
        <div
          class="absolute flex items-center justify-center {nodeWidth} h-10 px-3 rounded-lg border-2 border-white shadow text-white text-sm font-medium transform -translate-x-1/2 -translate-y-1/2 whitespace-nowrap {failed ? 'animate-pulse' : ''}"
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
  
  <!-- Scenario Explanation for Architecture Comparison -->
  {#if selectedSimulationType === 'architecture_comparison'}
    <div class="bg-white border rounded-lg p-6">
      <h3 class="text-xl font-semibold mb-4 text-center">📌 What's Happening: {scenarios[activeScenario]}</h3>
      
      {#if activeScenario === 'normal'}
        <div class="bg-green-50 border border-green-200 rounded-lg p-4">
          <h4 class="text-lg font-semibold text-green-800 mb-3">✅ Everything Working Normally</h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-white p-3 rounded border-l-4 border-red-400">
              <h5 class="font-medium text-red-700 mb-2">🔗 Traditional Way</h5>
              <p class="text-sm text-neutral-700">Each service calls the next service directly. Like a chain - each link depends on the next one working.</p>
            </div>
            <div class="bg-white p-3 rounded border-l-4 border-blue-400">
              <h5 class="font-medium text-blue-700 mb-2">🚀 Message Broker Way</h5>
              <p class="text-sm text-neutral-700">Services send messages to a smart queue. The queue delivers messages when services are ready.</p>
            </div>
          </div>
        </div>
      {:else if activeScenario === 'failure'}
        <div class="bg-red-50 border border-red-200 rounded-lg p-4">
          <h4 class="text-lg font-semibold text-red-800 mb-3">💥 Payment Service is Down!</h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-white p-3 rounded border-l-4 border-red-500">
              <h5 class="font-medium text-red-700 mb-2">😰 Traditional Problem</h5>
              <p class="text-sm text-neutral-700"><strong>Everything breaks!</strong> When payment fails, the whole order process stops. Users can't place orders at all.</p>
              <div class="mt-2 text-xs text-red-600">❌ Order → Payment (FAILS) → Everything stops</div>
            </div>
            <div class="bg-white p-3 rounded border-l-4 border-green-500">
              <h5 class="font-medium text-green-700 mb-2">😊 Message Broker Solution</h5>
              <p class="text-sm text-neutral-700"><strong>Keep working!</strong> Orders go to the queue. When payment service comes back online, it processes all waiting orders.</p>
              <div class="mt-2 text-xs text-green-600">✅ Order → Queue → Payment (when ready)</div>
            </div>
          </div>
        </div>
      {:else if activeScenario === 'highLoad'}
        <div class="bg-orange-50 border border-orange-200 rounded-lg p-4">
          <h4 class="text-lg font-semibold text-orange-800 mb-3">🚀 Lots of Users Right Now!</h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-white p-3 rounded border-l-4 border-red-500">
              <h5 class="font-medium text-red-700 mb-2">😓 Traditional Struggle</h5>
              <p class="text-sm text-neutral-700"><strong>Everything slows down!</strong> Each service waits for the previous one. Like a traffic jam - everyone gets stuck.</p>
              <div class="mt-2 text-xs text-red-600">🐌 Slow → Slower → Even Slower</div>
            </div>
            <div class="bg-white p-3 rounded border-l-4 border-green-500">
              <h5 class="font-medium text-green-700 mb-2">😎 Message Broker Handles It</h5>
              <p class="text-sm text-neutral-700"><strong>Smooth sailing!</strong> Messages wait in line. Each service works at its own speed without blocking others.</p>
              <div class="mt-2 text-xs text-green-600">📦 Queue manages the traffic flow</div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}
  
  <!-- Message Log -->
  <div class="bg-white border rounded-lg">
    <div class="p-4 border-b bg-neutral-50">
      <h3 class="font-semibold">Message Log</h3>
      <p class="text-sm text-neutral-600">Real-time messages for {simulationTypes[selectedSimulationType]}</p>
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
    {#if selectedSimulationType === 'architecture_comparison'}
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 text-sm mb-3">
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
      <div class="grid grid-cols-3 gap-4 text-sm border-t pt-3">
        <div class="flex items-center space-x-2">
          <span>💥</span>
          <span>Failed Request</span>
        </div>
        <div class="flex items-center space-x-2">
          <span>🐌</span>
          <span>Slow Processing</span>
        </div>
        <div class="flex items-center space-x-2">
          <span>📦</span>
          <span>Queued for Later</span>
        </div>
      </div>
    {:else}
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
    {/if}
  </div>
</div>

<style>
  .message-packet {
    filter: drop-shadow(0 0 5px rgba(255, 68, 68, 1)) 
            drop-shadow(0 0 10px rgba(255, 107, 53, 0.8)) 
            drop-shadow(0 0 15px rgba(255, 165, 0, 0.6));
  }
  
  .message-dot {
    filter: drop-shadow(0 0 3px rgba(0, 0, 0, 0.3));
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