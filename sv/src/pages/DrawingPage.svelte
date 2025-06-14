<script>
  import CollaborativeDrawing from '../components/CollaborativeDrawing.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-purple-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-purple-900 mb-2">Collaborative Drawing Board</h1>
    <p class="text-purple-700">Real-time collaborative drawing using RabbitMQ fanout exchange to broadcast drawing events to all connected users.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">Architecture Overview</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">1. User Input</h3>
        <p class="text-blue-700">Drawing events & cursor moves</p>
      </div>
      <div class="bg-purple-50 p-3 rounded">
        <h3 class="font-medium text-purple-900">2. RabbitMQ Fanout</h3>
        <p class="text-purple-700">Broadcasts to all clients</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">3. WebSocket</h3>
        <p class="text-yellow-700">Real-time event delivery</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">4. Canvas Sync</h3>
        <p class="text-green-700">Instant visual updates</p>
      </div>
    </div>
    
    <div class="mt-4">
      <h3 class="text-md font-medium mb-3">Collaborative Drawing Architecture</h3>
      <svg width="100%" height="180" viewBox="0 0 900 180" class="border rounded bg-gray-50">
        <defs>
          <marker id="arrowhead5" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151" />
          </marker>
        </defs>
        
        <!-- User 1 -->
        <rect x="20" y="20" width="70" height="40" rx="6" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="55" y="37" text-anchor="middle" fill="white" font-size="10" font-weight="bold">User 1</text>
        <text x="55" y="50" text-anchor="middle" fill="white" font-size="8">Drawing</text>
        
        <!-- User 2 -->
        <rect x="20" y="70" width="70" height="40" rx="6" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="55" y="87" text-anchor="middle" fill="white" font-size="10" font-weight="bold">User 2</text>
        <text x="55" y="100" text-anchor="middle" fill="white" font-size="8">Drawing</text>
        
        <!-- User 3 -->
        <rect x="20" y="120" width="70" height="40" rx="6" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="55" y="137" text-anchor="middle" fill="white" font-size="10" font-weight="bold">User 3</text>
        <text x="55" y="150" text-anchor="middle" fill="white" font-size="8">Drawing</text>
        
        <!-- WebSocket Server -->
        <rect x="150" y="70" width="100" height="50" rx="8" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="200" y="90" text-anchor="middle" fill="white" font-size="11" font-weight="bold">WebSocket</text>
        <text x="200" y="105" text-anchor="middle" fill="white" font-size="9">Server</text>
        
        <!-- Fanout Exchange -->
        <polygon points="320,65 370,80 370,110 320,125 300,95" fill="#8B5CF6" stroke="#7C3AED" stroke-width="2"/>
        <text x="335" y="90" text-anchor="middle" fill="white" font-size="10" font-weight="bold">Fanout</text>
        <text x="335" y="105" text-anchor="middle" fill="white" font-size="8">Exchange</text>
        
        <!-- Drawing Events Queue 1 -->
        <rect x="450" y="20" width="90" height="35" rx="4" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="495" y="35" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Canvas Queue 1</text>
        <text x="495" y="47" text-anchor="middle" fill="white" font-size="8">Draw Events</text>
        
        <!-- Drawing Events Queue 2 -->
        <rect x="450" y="70" width="90" height="35" rx="4" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="495" y="85" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Canvas Queue 2</text>
        <text x="495" y="97" text-anchor="middle" fill="white" font-size="8">Draw Events</text>
        
        <!-- Drawing Events Queue 3 -->
        <rect x="450" y="120" width="90" height="35" rx="4" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="495" y="135" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Canvas Queue 3</text>
        <text x="495" y="147" text-anchor="middle" fill="white" font-size="8">Draw Events</text>
        
        <!-- Canvas Sync 1 -->
        <rect x="600" y="20" width="80" height="35" rx="4" fill="#EF4444" stroke="#DC2626" stroke-width="2"/>
        <text x="640" y="35" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Canvas 1</text>
        <text x="640" y="47" text-anchor="middle" fill="white" font-size="8">Live Update</text>
        
        <!-- Canvas Sync 2 -->
        <rect x="600" y="70" width="80" height="35" rx="4" fill="#EF4444" stroke="#DC2626" stroke-width="2"/>
        <text x="640" y="85" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Canvas 2</text>
        <text x="640" y="97" text-anchor="middle" fill="white" font-size="8">Live Update</text>
        
        <!-- Canvas Sync 3 -->
        <rect x="600" y="120" width="80" height="35" rx="4" fill="#EF4444" stroke="#DC2626" stroke-width="2"/>
        <text x="640" y="135" text-anchor="middle" fill="white" font-size="9" font-weight="bold">Canvas 3</text>
        <text x="640" y="147" text-anchor="middle" fill="white" font-size="8">Live Update</text>
        
        <!-- Arrows from Users to WebSocket -->
        <line x1="90" y1="40" x2="140" y2="85" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <line x1="90" y1="90" x2="140" y2="95" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <line x1="90" y1="140" x2="140" y2="105" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        
        <!-- Arrow from WebSocket to Exchange -->
        <line x1="250" y1="95" x2="290" y2="95" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <text x="270" y="90" text-anchor="middle" fill="#374151" font-size="8">broadcast</text>
        
        <!-- Arrows from Exchange to Queues -->
        <line x1="370" y1="80" x2="440" y2="40" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <line x1="370" y1="95" x2="440" y2="87" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <line x1="370" y1="110" x2="440" y2="135" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        
        <!-- Arrows from Queues to Canvas -->
        <line x1="540" y1="37" x2="590" y2="37" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <line x1="540" y1="87" x2="590" y2="87" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        <line x1="540" y1="137" x2="590" y2="137" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead5)"/>
        
        <!-- Real-time Label -->
        <rect x="730" y="80" width="120" height="30" rx="4" fill="#F3F4F6" stroke="#D1D5DB" stroke-width="1"/>
        <text x="790" y="92" text-anchor="middle" fill="#374151" font-size="10" font-weight="bold">Real-time Sync</text>
        <text x="790" y="104" text-anchor="middle" fill="#6B7280" font-size="8">All users see changes</text>
      </svg>
    </div>
  </div>
  
  <CollaborativeDrawing {ws} {connected} />
</div>