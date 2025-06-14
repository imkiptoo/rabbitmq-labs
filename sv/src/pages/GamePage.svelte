<script>
  import RaceGame from '../components/RaceGame.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-yellow-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-yellow-900 mb-2">Race to 100 Game</h1>
    <p class="text-yellow-700">Demonstrates fanout exchange pattern with pub/sub messaging for real-time multiplayer updates.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">How it works</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">1. Game Server</h3>
        <p class="text-blue-700">Publishes score updates</p>
      </div>
      <div class="bg-purple-50 p-3 rounded">
        <h3 class="font-medium text-purple-900">2. Fanout Exchange</h3>
        <p class="text-purple-700">Broadcasts to all queues</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">3. Player Queues</h3>
        <p class="text-yellow-700">Each player has own queue</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">4. All Players</h3>
        <p class="text-green-700">Receive live updates</p>
      </div>
    </div>
    
    <div class="mt-4">
      <h3 class="text-md font-medium mb-3">Fanout Exchange Diagram</h3>
      <svg width="100%" height="200" viewBox="0 0 800 200" class="border rounded bg-gray-50">
        <defs>
          <marker id="arrowhead3" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151" />
          </marker>
        </defs>
        
        <!-- Game Server -->
        <rect x="20" y="80" width="90" height="50" rx="8" fill="#3B82F6" stroke="#1E40AF" stroke-width="2"/>
        <text x="65" y="100" text-anchor="middle" fill="white" font-size="12" font-weight="bold">Game Server</text>
        <text x="65" y="115" text-anchor="middle" fill="white" font-size="10">Score Updates</text>
        
        <!-- Arrow to Exchange -->
        <line x1="110" y1="105" x2="170" y2="105" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        <text x="140" y="100" text-anchor="middle" fill="#374151" font-size="10">publish</text>
        
        <!-- Fanout Exchange -->
        <polygon points="190,70 240,85 240,125 190,140 170,105" fill="#8B5CF6" stroke="#7C3AED" stroke-width="2"/>
        <text x="205" y="100" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Fanout</text>
        <text x="205" y="115" text-anchor="middle" fill="white" font-size="9">Exchange</text>
        
        <!-- Player 1 Queue -->
        <rect x="350" y="30" width="80" height="40" rx="6" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="390" y="45" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Player 1</text>
        <text x="390" y="58" text-anchor="middle" fill="white" font-size="9">Queue</text>
        
        <!-- Player 2 Queue -->
        <rect x="350" y="80" width="80" height="40" rx="6" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="390" y="95" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Player 2</text>
        <text x="390" y="108" text-anchor="middle" fill="white" font-size="9">Queue</text>
        
        <!-- Player 3 Queue -->
        <rect x="350" y="130" width="80" height="40" rx="6" fill="#F59E0B" stroke="#D97706" stroke-width="2"/>
        <text x="390" y="145" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Player 3</text>
        <text x="390" y="158" text-anchor="middle" fill="white" font-size="9">Queue</text>
        
        <!-- Arrows from Exchange to Queues -->
        <line x1="240" y1="90" x2="340" y2="50" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        <line x1="240" y1="105" x2="340" y2="100" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        <line x1="240" y1="120" x2="340" y2="150" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        
        <!-- Player 1 Consumer -->
        <rect x="500" y="30" width="80" height="40" rx="6" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="540" y="45" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Player 1</text>
        <text x="540" y="58" text-anchor="middle" fill="white" font-size="9">Client</text>
        
        <!-- Player 2 Consumer -->
        <rect x="500" y="80" width="80" height="40" rx="6" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="540" y="95" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Player 2</text>
        <text x="540" y="108" text-anchor="middle" fill="white" font-size="9">Client</text>
        
        <!-- Player 3 Consumer -->
        <rect x="500" y="130" width="80" height="40" rx="6" fill="#10B981" stroke="#059669" stroke-width="2"/>
        <text x="540" y="145" text-anchor="middle" fill="white" font-size="11" font-weight="bold">Player 3</text>
        <text x="540" y="158" text-anchor="middle" fill="white" font-size="9">Client</text>
        
        <!-- Arrows from Queues to Consumers -->
        <line x1="430" y1="50" x2="490" y2="50" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        <line x1="430" y1="100" x2="490" y2="100" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        <line x1="430" y1="150" x2="490" y2="150" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead3)"/>
        
        <!-- Broadcast Label -->
        <text x="290" y="60" text-anchor="middle" fill="#374151" font-size="10" font-weight="bold">Broadcast</text>
        <text x="290" y="160" text-anchor="middle" fill="#374151" font-size="10">to All Players</text>
        
        <!-- Live Updates Box -->
        <rect x="650" y="85" width="120" height="35" rx="6" fill="#EF4444" stroke="#DC2626" stroke-width="2" stroke-dasharray="5,5"/>
        <text x="710" y="100" text-anchor="middle" fill="#EF4444" font-size="11" font-weight="bold">Real-time Updates</text>
        <text x="710" y="113" text-anchor="middle" fill="#EF4444" font-size="9">All see same scores</text>
      </svg>
    </div>
  </div>
  
  <RaceGame {ws} {connected} />
</div>