<script>
  import MessageLogger from '../components/MessageLogger.svelte';
  import SinglePatternFlowSimulator from '../components/SinglePatternFlowSimulator.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-3">
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
  </div>
  
  <MessageLogger {ws} {connected} />
</div>