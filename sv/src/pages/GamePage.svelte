<script>
  import RaceGame from '../components/RaceGame.svelte';
  import SinglePatternFlowSimulator from '../components/SinglePatternFlowSimulator.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-3">
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
  </div>
  
  <RaceGame {ws} {connected} />
</div>