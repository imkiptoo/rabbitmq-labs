<script>
  import NumberDoubler from '../components/NumberDoubler.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-green-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-green-900 mb-2">Number Doubler Workers</h1>
    <p class="text-green-700">Demonstrates work queue pattern with multiple competing consumers for load balancing.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">How it works</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">1. Producer</h3>
        <p class="text-blue-700">Sends work items to queue</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">2. Work Queue</h3>
        <p class="text-yellow-700">Distributes work among workers</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">3. Workers</h3>
        <p class="text-green-700">3 workers compete for tasks</p>
      </div>
      <div class="bg-purple-50 p-3 rounded">
        <h3 class="font-medium text-purple-900">4. Load Balancing</h3>
        <p class="text-purple-700">Automatic work distribution</p>
      </div>
    </div>
    
    <div class="mt-4">
      <h3 class="text-md font-medium mb-3">Work Queue Diagram</h3>
      <svg width="100%" height="180" viewBox="0 0 700 180" class="border rounded bg-gray-50">
        <defs>
          <marker id="arrowhead2" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151" />
          </marker>
        </defs>
        
        <!-- Producer -->
        <rect x="20" y="70" width="80" height="50" rx="8" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="60" y="90" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Producer</text>
        <text x="60" y="105" text-anchor="middle" fill="white" font-size="10">Work Items</text>
        
        <!-- Arrow to Queue -->
        <line x1="100" y1="95" x2="160" y2="95" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead2)"/>
        <text x="130" y="90" text-anchor="middle" fill="#374151" font-size="10">publish</text>
        
        <!-- Work Queue -->
        <rect x="180" y="70" width="80" height="50" rx="8" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="220" y="90" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Work Queue</text>
        <text x="220" y="105" text-anchor="middle" fill="white" font-size="10">Load Balance</text>
        
        <!-- Worker 1 -->
        <rect x="350" y="20" width="70" height="40" rx="6" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="385" y="37" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Worker 1</text>
        <text x="385" y="50" text-anchor="middle" fill="white" font-size="9">Available</text>
        
        <!-- Worker 2 -->
        <rect x="350" y="70" width="70" height="40" rx="6" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="385" y="87" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Worker 2</text>
        <text x="385" y="100" text-anchor="middle" fill="white" font-size="9">Busy</text>
        
        <!-- Worker 3 -->
        <rect x="350" y="120" width="70" height="40" rx="6" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="385" y="137" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Worker 3</text>
        <text x="385" y="150" text-anchor="middle" fill="white" font-size="9">Available</text>
        
        <!-- Arrows to Workers -->
        <line x1="260" y1="85" x2="340" y2="45" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead2)"/>
        <line x1="260" y1="95" x2="340" y2="90" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead2)"/>
        <line x1="260" y1="105" x2="340" y2="135" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead2)"/>
        
        <!-- Round Robin Label -->
        <text x="300" y="55" text-anchor="middle" fill="#374151" font-size="10" font-weight="bold">Round Robin</text>
        <text x="300" y="125" text-anchor="middle" fill="#374151" font-size="10">Distribution</text>
        
        <!-- Results -->
        <rect x="500" y="70" width="80" height="50" rx="8" fill="#8B5CF6" stroke="#7C3AED" stroke-width="2"/>
        <text x="540" y="90" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Results</text>
        <text x="540" y="105" text-anchor="middle" fill="white" font-size="10">Doubled Numbers</text>
        
        <!-- Arrow to Results -->
        <line x1="430" y1="95" x2="490" y2="95" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead2)"/>
      </svg>
    </div>
  </div>
  
  <NumberDoubler {ws} {connected} />
</div>