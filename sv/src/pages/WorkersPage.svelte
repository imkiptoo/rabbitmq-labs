<script>
  import NumberDoubler from '../components/NumberDoubler.svelte';
  import SinglePatternFlowSimulator from '../components/SinglePatternFlowSimulator.svelte';
  
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
  </div>
  
  <NumberDoubler {ws} {connected} />
  
  <SinglePatternFlowSimulator demoType="workers" {ws} {connected} />
</div>