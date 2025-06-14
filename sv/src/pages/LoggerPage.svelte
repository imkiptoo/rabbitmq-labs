<script>
  import MessageLogger from '../components/MessageLogger.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-blue-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-blue-900 mb-2">Message Logger</h1>
    <p class="text-blue-700">Demonstrates the basic producer-consumer pattern with simple message queuing.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">How it works</h2>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">1. Producer</h3>
        <p class="text-blue-700">Sends messages to the "messages" queue</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">2. Queue</h3>
        <p class="text-yellow-700">Stores messages until consumed</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">3. Consumer</h3>
        <p class="text-green-700">Receives and processes messages</p>
      </div>
    </div>
    
    <div class="mt-4">
      <h3 class="text-md font-medium mb-3">Message Flow Diagram</h3>
      <svg width="100%" height="120" viewBox="0 0 600 120" class="border rounded bg-gray-50">
        <defs>
          <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151" />
          </marker>
        </defs>
        
        <!-- Producer -->
        <rect x="20" y="30" width="80" height="60" rx="8" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="60" y="55" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Producer</text>
        <text x="60" y="70" text-anchor="middle" fill="white" font-size="10">Sends Messages</text>
        
        <!-- Arrow 1 -->
        <line x1="100" y1="60" x2="160" y2="60" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead)"/>
        <text x="130" y="55" text-anchor="middle" fill="#374151" font-size="10">publish</text>
        
        <!-- Queue -->
        <rect x="180" y="30" width="80" height="60" rx="8" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="220" y="55" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Queue</text>
        <text x="220" y="70" text-anchor="middle" fill="white" font-size="10">"messages"</text>
        
        <!-- Arrow 2 -->
        <line x1="260" y1="60" x2="320" y2="60" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead)"/>
        <text x="290" y="55" text-anchor="middle" fill="#374151" font-size="10">consume</text>
        
        <!-- Consumer -->
        <rect x="340" y="30" width="80" height="60" rx="8" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="380" y="55" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Consumer</text>
        <text x="380" y="70" text-anchor="middle" fill="white" font-size="10">Processes</text>
      </svg>
    </div>
  </div>
  
  <MessageLogger {ws} {connected} />
</div>