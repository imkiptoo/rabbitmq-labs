<script>
  import StatusChecker from '../components/StatusChecker.svelte';
  import SinglePatternFlowSimulator from '../components/SinglePatternFlowSimulator.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-3">
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
  </div>
  
  <StatusChecker {ws} {connected} />
</div>