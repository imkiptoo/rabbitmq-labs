<script>
  export let ws;
  export let connected;

  let number = '';
  let results = [];
  let loading = false;

  function handleWebSocketMessage(event) {
    const data = event.detail;
    if (data.demo_type === 'workers') {
      results = [...results, {
        worker_id: data.data.worker_id,
        task_id: data.data.task_id,
        original: data.data.original,
        result: data.data.result,
        processing_time: data.data.processing_time,
        timestamp: new Date().toLocaleTimeString(),
        id: Date.now() + Math.random()
      }];
    }
  }

  window.addEventListener('websocket-message', handleWebSocketMessage);

  async function submitNumber() {
    if (!number || loading) return;

    const numValue = parseInt(number);
    if (isNaN(numValue)) {
      alert('Please enter a valid number');
      return;
    }

    loading = true;
    try {
      const response = await fetch('http://localhost:3030/api/workers/submit', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ number: numValue }),
      });

      if (response.ok) {
        number = '';
      } else {
        console.error('Failed to submit number');
      }
    } catch (error) {
      console.error('Error submitting number:', error);
    } finally {
      loading = false;
    }
  }

  function handleKeyPress(event) {
    if (event.key === 'Enter') {
      event.preventDefault();
      submitNumber();
    }
  }

  function clearResults() {
    results = [];
  }
</script>

<div class="space-y-3">
  <div class="flex space-x-3 items-end">
    <div class="flex-1">
      <label for="number-input" class="block text-sm font-medium text-neutral-700 mb-2">
        Enter a number to double:
      </label>
      <input
        id="number-input"
        type="number"
        bind:value={number}
        on:keypress={handleKeyPress}
        placeholder="Enter any number..."
        class="w-full px-3 py-1 border border-neutral-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent"
        disabled={loading}
      />
    </div>

    <button
      on:click={submitNumber}
      disabled={!number || loading || !connected}
      class="px-3 py-1 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:bg-neutral-400 disabled:cursor-not-allowed flex items-center"
    >
      {#if loading}
        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        Processing...
      {:else}
        Submit Number
      {/if}
    </button>

    <button
      on:click={clearResults}
      class="px-3 py-1 bg-neutral-600 text-white rounded-md hover:bg-neutral-700"
    >
      Clear Results
    </button>
  </div>

  <div class="bg-neutral-50 p-4 rounded-lg">
    <div class="flex justify-between items-center mb-4">
      <h4 class="text-lg font-semibold text-neutral-800">Processing Results ({results.length})</h4>
      <div class="text-sm text-neutral-600">
        Status: {connected ? 'Connected' : 'Disconnected'}
      </div>
    </div>

    <div class="grid gap-4 max-px-3 py-1 overflow-y-auto">
      {#each results as result (result.id)}
        <div class="bg-white p-4 rounded border-l-4 border-green-500">
          <div class="flex justify-between items-start mb-2">
            <div class="flex items-center space-x-2">
              <span class="bg-green-100 text-green-800 text-xs px-2 py-1 rounded">
                Worker {result.worker_id}
              </span>
              <span class="text-neutral-600 text-sm">
                Task: {result.task_id.substring(0, 8)}...
              </span>
            </div>
            <span class="text-xs text-neutral-500">{result.timestamp}</span>
          </div>
          <div class="text-lg">
            <span class="text-neutral-700">{result.original}</span>
            <span class="mx-2 text-neutral-400">×2 =</span>
            <span class="font-bold text-green-600">{result.result}</span>
          </div>
          <div class="text-sm text-neutral-500 mt-1">
            Processing time: {result.processing_time}ms
          </div>
        </div>
      {:else}
        <div class="text-center text-neutral-500 py-8">
          No results yet. Submit a number to see workers in action!
        </div>
      {/each}
    </div>
  </div>
</div>