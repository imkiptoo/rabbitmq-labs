<script>
  import MessageFlowSimulator from '../components/MessageFlowSimulator.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-indigo-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-indigo-900 mb-2">Message Flow Simulator</h1>
    <p class="text-indigo-700">Visualize and understand how messages flow through different RabbitMQ patterns with animated diagrams.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">Features</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">Visual Diagrams</h3>
        <p class="text-blue-700">See producers, queues, exchanges, and consumers</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">Live Animation</h3>
        <p class="text-green-700">Watch messages move through the system</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">Real-time Data</h3>
        <p class="text-yellow-700">Shows actual messages from other demos</p>
      </div>
      <div class="bg-purple-50 p-3 rounded">
        <h3 class="font-medium text-purple-900">Interactive</h3>
        <p class="text-purple-700">Simulate messages and control playback</p>
      </div>
    </div>
    
    <div class="mt-4">
      <h3 class="text-md font-medium mb-3">Message Flow Visualization System</h3>
      <svg width="100%" height="160" viewBox="0 0 900 160" class="border rounded bg-neutral-50">
        <defs>
          <marker id="arrowhead6" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151" />
          </marker>
        </defs>
        
        <!-- Real Message Systems -->
        <rect x="20" y="20" width="120" height="30" rx="4" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="80" y="32" text-anchor="middle" fill="white" font-size="10" font-weight="bold">Logger Demo</text>
        <text x="80" y="43" text-anchor="middle" fill="white" font-size="8">Live Messages</text>
        
        <rect x="20" y="60" width="120" height="30" rx="4" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="80" y="72" text-anchor="middle" fill="white" font-size="10" font-weight="bold">Workers Demo</text>
        <text x="80" y="83" text-anchor="middle" fill="white" font-size="8">Live Processing</text>
        
        <rect x="20" y="100" width="120" height="30" rx="4" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="80" y="112" text-anchor="middle" fill="white" font-size="10" font-weight="bold">Game Demo</text>
        <text x="80" y="123" text-anchor="middle" fill="white" font-size="8">Live Updates</text>
        
        <!-- Data Collection -->
        <rect x="200" y="60" width="100" height="50" rx="8" fill="#8B5CF6" stroke="#7C3AED" stroke-width="2"/>
        <text x="250" y="80" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Data Collector</text>
        <text x="250" y="95" text-anchor="middle" fill="white" font-size="9">Captures All Events</text>
        
        <!-- Visualization Engine -->
        <rect x="360" y="60" width="120" height="50" rx="8" fill="#EF4444" stroke="#DC2626" stroke-width="2"/>
        <text x="420" y="80" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Viz Engine</text>
        <text x="420" y="95" text-anchor="middle" fill="white" font-size="9">Animated Diagrams</text>
        
        <!-- Pattern Displays -->
        <rect x="550" y="20" width="100" height="25" rx="4" fill="#6366F1" stroke="#4F46E5" stroke-width="1"/>
        <text x="600" y="30" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Queue Pattern</text>
        <text x="600" y="40" text-anchor="middle" fill="white" font-size="7">Producer → Queue → Consumer</text>
        
        <rect x="550" y="55" width="100" height="25" rx="4" fill="#6366F1" stroke="#4F46E5" stroke-width="1"/>
        <text x="600" y="65" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Work Queue</text>
        <text x="600" y="75" text-anchor="middle" fill="white" font-size="7">Load Balancing Workers</text>
        
        <rect x="550" y="90" width="100" height="25" rx="4" fill="#6366F1" stroke="#4F46E5" stroke-width="1"/>
        <text x="600" y="100" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Fanout Pattern</text>
        <text x="600" y="110" text-anchor="middle" fill="white" font-size="7">Broadcast to All</text>
        
        <rect x="550" y="125" width="100" height="25" rx="4" fill="#6366F1" stroke="#4F46E5" stroke-width="1"/>
        <text x="600" y="135" text-anchor="middle" fill="white" font-size="9" font-weight="bold">RPC Pattern</text>
        <text x="600" y="145" text-anchor="middle" fill="white" font-size="7">Request-Reply Flow</text>
        
        <!-- Interactive Controls -->
        <rect x="720" y="45" width="120" height="70" rx="6" fill="#F3F4F6" stroke="#D1D5DB" stroke-width="2"/>
        <text x="780" y="60" text-anchor="middle" fill="#374151" font-size="10" font-weight="bold">Interactive Controls</text>
        <text x="780" y="75" text-anchor="middle" fill="#6B7280" font-size="8">• Play/Pause Animation</text>
        <text x="780" y="88" text-anchor="middle" fill="#6B7280" font-size="8">• Speed Control</text>
        <text x="780" y="101" text-anchor="middle" fill="#6B7280" font-size="8">• Pattern Selection</text>
        
        <!-- Arrows -->
        <line x1="140" y1="40" x2="190" y2="70" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        <line x1="140" y1="75" x2="190" y2="85" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        <line x1="140" y1="115" x2="190" y2="100" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        
        <line x1="300" y1="85" x2="350" y2="85" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        <text x="325" y="80" text-anchor="middle" fill="#374151" font-size="8">process</text>
        
        <line x1="480" y1="70" x2="540" y2="50" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        <line x1="480" y1="85" x2="540" y2="85" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        <line x1="480" y1="100" x2="540" y2="120" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        
        <line x1="650" y1="85" x2="710" y2="85" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead6)"/>
        <text x="680" y="80" text-anchor="middle" fill="#374151" font-size="8">control</text>
      </svg>
    </div>
  </div>
  
  <MessageFlowSimulator {ws} {connected} />
</div>