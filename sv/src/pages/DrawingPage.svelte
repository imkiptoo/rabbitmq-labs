<script>
  import CollaborativeDrawing from '../components/CollaborativeDrawing.svelte';
  import SinglePatternFlowSimulator from '../components/SinglePatternFlowSimulator.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-3">
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
  </div>
  
  <CollaborativeDrawing {ws} {connected} />
</div>