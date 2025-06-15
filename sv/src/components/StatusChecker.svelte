<script>
  export let ws;
  export let connected;

  let loading = false;
  let statusHistory = [];

  function handleWebSocketMessage(event) {
    const data = event.detail;
    if (data.demo_type === 'rpc') {
      if (data.data.type === 'status_response') {
        const entry = {
          id: Date.now(),
          timestamp: new Date().toLocaleTimeString(),
          request: data.data.request,
          response: data.data.response,
          success: true
        };
        statusHistory = [entry, ...statusHistory];
      } else if (data.data.type === 'status_error') {
        const entry = {
          id: Date.now(),
          timestamp: new Date().toLocaleTimeString(),
          error: data.data.error,
          success: false
        };
        statusHistory = [entry, ...statusHistory];
      }
    }
  }

  window.addEventListener('websocket-message', handleWebSocketMessage);

  async function checkStatus() {
    if (loading) return;

    loading = true;
    try {
      const response = await fetch('http://localhost:3030/api/rpc/status', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({}),
      });

      if (response.ok) {
        const data = await response.json();
        const entry = {
          id: Date.now(),
          timestamp: new Date().toLocaleTimeString(),
          response: data,
          success: data.success
        };
        statusHistory = [entry, ...statusHistory];
      } else {
        const entry = {
          id: Date.now(),
          timestamp: new Date().toLocaleTimeString(),
          error: 'HTTP Error: ' + response.status,
          success: false
        };
        statusHistory = [entry, ...statusHistory];
      }
    } catch (error) {
      const entry = {
        id: Date.now(),
        timestamp: new Date().toLocaleTimeString(),
        error: 'Network Error: ' + error.message,
        success: false
      };
      statusHistory = [entry, ...statusHistory];
    } finally {
      loading = false;
    }
  }

  function clearHistory() {
    statusHistory = [];
  }
</script>

<div class="space-y-6">
  <div class="bg-orange-50 p-4 rounded-lg">
    <h3 class="text-lg font-semibold text-orange-800 mb-2">Task Status Checker Demo</h3>
    <p class="text-orange-700">
      This demo shows RabbitMQ RPC (request/reply) pattern. The client sends a request and waits for a response from the server.
    </p>
  </div>

  <div class="text-center space-y-4">
    <button
      on:click={checkStatus}
      disabled={loading || !connected}
      class="px-8 py-3 bg-orange-600 text-white rounded-md hover:bg-orange-700 disabled:bg-neutral-400 disabled:cursor-not-allowed text-lg font-semibold flex items-center justify-center mx-auto"
    >
      {#if loading}
        <svg class="animate-spin -ml-1 mr-3 h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        Checking Status...
      {:else}
        Check Server Status
      {/if}
    </button>

    {#if !connected}
      <p class="text-red-600 text-sm">WebSocket not connected. Please wait...</p>
    {/if}

    <div class="flex justify-center">
      <button
        on:click={clearHistory}
        class="px-3 py-2 bg-neutral-600 text-white rounded-md hover:bg-neutral-700 text-sm"
      >
        Clear History
      </button>
    </div>
  </div>

  <div class="bg-neutral-50 p-4 rounded-lg">
    <div class="flex justify-between items-center mb-4">
      <h4 class="text-lg font-semibold text-neutral-800">Status Check History ({statusHistory.length})</h4>
      <div class="text-sm text-neutral-600">
        Status: {connected ? 'Connected' : 'Disconnected'}
      </div>
    </div>

    <div class="space-y-3 max-h-96 overflow-y-auto">
      {#each statusHistory as entry (entry.id)}
        <div class="bg-white p-4 rounded border-l-4 {entry.success ? 'border-green-500' : 'border-red-500'}">
          <div class="flex justify-between items-start mb-2">
            <div class="flex items-center space-x-2">
              <span class="px-2 py-1 rounded text-xs {
                entry.success 
                  ? 'bg-green-100 text-green-800' 
                  : 'bg-red-100 text-red-800'
              }">
                {entry.success ? 'SUCCESS' : 'ERROR'}
              </span>
            </div>
            <span class="text-xs text-neutral-500">{entry.timestamp}</span>
          </div>

          {#if entry.success && entry.response}
            <div class="space-y-2">
              <div class="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span class="font-medium text-neutral-600">Timestamp:</span>
                  <p class="text-neutral-800">{new Date(entry.response.timestamp).toLocaleString()}</p>
                </div>
                <div>
                  <span class="font-medium text-neutral-600">Status:</span>
                  <p class="text-neutral-800">{entry.response.status}</p>
                </div>
              </div>
              <div>
                <span class="font-medium text-neutral-600">Server Info:</span>
                <p class="text-neutral-800">{entry.response.server_info}</p>
              </div>
            </div>
          {:else if entry.error}
            <div class="text-red-600">
              <span class="font-medium">Error:</span> {entry.error}
            </div>
          {/if}
        </div>
      {:else}
        <div class="text-center text-neutral-500 py-8">
          No status checks yet. Click the button above to check server status!
        </div>
      {/each}
    </div>
  </div>
</div>