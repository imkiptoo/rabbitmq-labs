<script>
  import StatusChecker from '../components/StatusChecker.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-red-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-red-900 mb-2">RPC Status Checker</h1>
    <p class="text-red-700">Demonstrates request-reply pattern with correlation IDs for synchronous-style communication.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">How it works</h2>
    <div class="grid grid-cols-1 md:grid-cols-5 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">1. Client</h3>
        <p class="text-blue-700">Sends RPC request</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">2. Request Queue</h3>
        <p class="text-yellow-700">Holds requests</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">3. Server</h3>
        <p class="text-green-700">Processes request</p>
      </div>
      <div class="bg-purple-50 p-3 rounded">
        <h3 class="font-medium text-purple-900">4. Reply Queue</h3>
        <p class="text-purple-700">Temporary response queue</p>
      </div>
      <div class="bg-indigo-50 p-3 rounded">
        <h3 class="font-medium text-indigo-900">5. Correlation ID</h3>
        <p class="text-indigo-700">Matches request to reply</p>
      </div>
    </div>
    
    <div class="mt-4">
      <h3 class="text-md font-medium mb-3">RPC Pattern Diagram</h3>
      <svg width="100%" height="160" viewBox="0 0 900 160" class="border rounded bg-gray-50">
        <defs>
          <marker id="arrowhead4" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151" />
          </marker>
          <marker id="arrowhead4-return" markerWidth="10" markerHeight="7" refX="1" refY="3.5" orient="auto">
            <polygon points="10 0, 0 3.5, 10 7" fill="#059669" />
          </marker>
        </defs>
        
        <!-- Client -->
        <rect x="20" y="30" width="80" height="40" rx="8" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="60" y="47" text-anchor="middle" fill="white" font-size="11" font-weight="bold">RPC Client</text>
        <text x="60" y="60" text-anchor="middle" fill="white" font-size="9">Sends Request</text>
        
        <!-- Request Queue -->
        <rect x="150" y="30" width="80" height="40" rx="6" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="190" y="47" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Request</text>
        <text x="190" y="60" text-anchor="middle" fill="white" font-size="9">Queue</text>
        
        <!-- Server -->
        <rect x="280" y="30" width="80" height="40" rx="8" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="320" y="47" text-anchor="middle" fill="white" font-size="11" font-weight="bold">RPC Server</text>
        <text x="320" y="60" text-anchor="middle" fill="white" font-size="9">Processes</text>
        
        <!-- Reply Queue -->
        <rect x="410" y="30" width="80" height="40" rx="6" fill="#8B5CF6" stroke="#7C3AED" stroke-width="2"/>
        <text x="450" y="47" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Reply</text>
        <text x="450" y="60" text-anchor="middle" fill="white" font-size="9">Queue</text>
        
        <!-- Correlation ID -->
        <rect x="550" y="30" width="100" height="40" rx="6" fill="#EF4444" stroke="#DC2626" stroke-width="2"/>
        <text x="600" y="47" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Correlation ID</text>
        <text x="600" y="60" text-anchor="middle" fill="white" font-size="9">UUID-123</text>
        
        <!-- Request Flow Arrows -->
        <line x1="100" y1="45" x2="140" y2="45" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead4)"/>
        <text x="120" y="40" text-anchor="middle" fill="#374151" font-size="9">1. Request</text>
        
        <line x1="230" y1="45" x2="270" y2="45" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead4)"/>
        <text x="250" y="40" text-anchor="middle" fill="#374151" font-size="9">2. Process</text>
        
        <line x1="360" y1="45" x2="400" y2="45" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead4)"/>
        <text x="380" y="40" text-anchor="middle" fill="#374151" font-size="9">3. Reply</text>
        
        <!-- Reply Flow (curved back) -->
        <path d="M 450 70 Q 325 110 60 80" stroke="#059669" stroke-width="2" fill="none" marker-end="url(#arrowhead4-return)"/>
        <text x="325" y="100" text-anchor="middle" fill="#059669" font-size="9">4. Response with Correlation ID</text>
        
        <!-- Correlation ID Connection (dotted) -->
        <line x1="600" y1="30" x2="600" y2="15" stroke="#EF4444" stroke-width="1" stroke-dasharray="3,3"/>
        <line x1="60" y1="15" x2="600" y2="15" stroke="#EF4444" stroke-width="1" stroke-dasharray="3,3"/>
        <line x1="60" y1="15" x2="60" y2="30" stroke="#EF4444" stroke-width="1" stroke-dasharray="3,3"/>
        <text x="330" y="10" text-anchor="middle" fill="#EF4444" font-size="9">Matches Request to Response</text>
        
        <!-- Status Example -->
        <rect x="720" y="20" width="140" height="60" rx="6" fill="#F3F4F6" stroke="#D1D5DB" stroke-width="1"/>
        <text x="790" y="35" text-anchor="middle" fill="#374151" font-size="10" font-weight="bold">Example Request</text>
        <text x="790" y="48" text-anchor="middle" fill="#6B7280" font-size="9">Check server status</text>
        <text x="790" y="61" text-anchor="middle" fill="#059669" font-size="9">→ "Server is healthy"</text>
      </svg>
    </div>
  </div>
  
  <StatusChecker {ws} {connected} />
</div>