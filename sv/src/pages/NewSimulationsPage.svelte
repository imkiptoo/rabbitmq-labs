<script>
  import { onMount } from "svelte";
  import * as d3 from "d3";

  export let ws;
  export let connected;

  let svgElement;
  let svg;
  let activeMessages = [];
  let animationSpeed = 800;
  let messageCount = 0;
  let queueToConsumerTransitions = [];
  let queueToConsumerTimeout = null;

  const fanoutDemo = {
    name: "Fanout Exchange Pattern",
    description:
      "Producer → fanout exchange → multiple queues → consumers (broadcast to all)",
    nodes: [
      {
        id: "producer",
        type: "producer",
        x: 100,
        y: 200,
        label: "Message Producer",
      },
      {
        id: "exchange",
        type: "exchange",
        x: 300,
        y: 200,
        label: "Fanout Exchange",
      },
      { id: "queue1", type: "queue", x: 500, y: 120, label: "Queue A" },
      { id: "queue2", type: "queue", x: 500, y: 200, label: "Queue B" },
      { id: "queue3", type: "queue", x: 500, y: 280, label: "Queue C" },
      {
        id: "consumer1",
        type: "consumer",
        x: 700,
        y: 120,
        label: "Consumer A",
      },
      {
        id: "consumer2",
        type: "consumer",
        x: 700,
        y: 200,
        label: "Consumer B",
      },
      {
        id: "consumer3",
        type: "consumer",
        x: 700,
        y: 280,
        label: "Consumer C",
      },
    ],
    connections: [
      { from: "producer", to: "exchange" },
      { from: "exchange", to: "queue1" },
      { from: "exchange", to: "queue2" },
      { from: "exchange", to: "queue3" },
      { from: "queue1", to: "consumer1" },
      { from: "queue2", to: "consumer2" },
      { from: "queue3", to: "consumer3" },
    ],
  };

  let currentDemo = fanoutDemo;

  onMount(() => {
    if (typeof window !== "undefined") {
      window.addEventListener("websocket-message", handleWebSocketMessage);
      initializeD3Visualization();
      return () => {
        window.removeEventListener("websocket-message", handleWebSocketMessage);
      };
    }
  });

  function initializeD3Visualization() {
    const width = 900;
    const height = 400;

    svg = d3
      .select(svgElement)
      .attr("width", width)
      .attr("height", height)
      .attr("viewBox", `0 0 ${width} ${height}`);

    // Create gradient definitions
    const defs = svg.append("defs");

    // Message gradients
    const messageGradient = defs
      .append("radialGradient")
      .attr("id", "messageGradient")
      .attr("cx", "50%")
      .attr("cy", "50%")
      .attr("r", "50%");
    messageGradient
      .append("stop")
      .attr("offset", "0%")
      .attr("stop-color", "#FCA5A5")
      .attr("stop-opacity", 1);
    messageGradient
      .append("stop")
      .attr("offset", "100%")
      .attr("stop-color", "#EF4444")
      .attr("stop-opacity", 1);

    const fanoutGradient = defs
      .append("radialGradient")
      .attr("id", "fanoutMessageGradient")
      .attr("cx", "50%")
      .attr("cy", "50%")
      .attr("r", "50%");
    fanoutGradient
      .append("stop")
      .attr("offset", "0%")
      .attr("stop-color", "#A7F3D0")
      .attr("stop-opacity", 1);
    fanoutGradient
      .append("stop")
      .attr("offset", "100%")
      .attr("stop-color", "#10B981")
      .attr("stop-opacity", 1);

    // Arrow markers
    defs
      .append("marker")
      .attr("id", "arrowhead")
      .attr("markerWidth", 10)
      .attr("markerHeight", 7)
      .attr("refX", 5)
      .attr("refY", 3.5)
      .attr("orient", "auto")
      .append("polygon")
      .attr("points", "0 0, 5 3.5, 0 7")
      .attr("fill", "#d0d0d0");

    defs
      .append("marker")
      .attr("id", "arrowhead-active")
      .attr("markerWidth", 10)
      .attr("markerHeight", 7)
      .attr("refX", 5)
      .attr("refY", 3.5)
      .attr("orient", "auto")
      .append("polygon")
      .attr("points", "0 0, 5 3.5, 0 7")
      .attr("fill", "#FF4444");

    // Create groups for different elements in proper z-order (back to front)
    svg.append("g").attr("class", "connections");
    svg.append("g").attr("class", "nodes");
    svg.append("g").attr("class", "connection-highlights");
    svg.append("g").attr("class", "messages");

    drawConnections();
    drawNodes();
  }

  function drawConnections() {
    const connectionsGroup = svg.select(".connections");

    connectionsGroup
      .selectAll(".connection")
      .data(currentDemo.connections)
      .enter()
      .append("path")
      .attr("class", "connection")
      .attr("d", (d) => getConnectionPath(d.from, d.to))
      .attr("stroke", "#d0d0d0")
      .attr("stroke-width", 2)
      .attr("fill", "none")
      .attr("marker-end", "url(#arrowhead)");
  }

  function drawNodes() {
    const nodesGroup = svg.select(".nodes");

    const nodeElements = nodesGroup
      .selectAll(".node")
      .data(currentDemo.nodes)
      .enter()
      .append("g")
      .attr("class", "node")
      .attr("transform", (d) => `translate(${d.x}, ${d.y})`);

    // Draw node shapes based on type
    nodeElements.each(function (d) {
      const node = d3.select(this);

      if (d.type === "producer") {
        // Producer: Blue Square (same height as queue/consumer)
        node
          .append("rect")
          .attr("x", -15)
          .attr("y", -15)
          .attr("width", 30)
          .attr("height", 30)
          .attr("rx", 3)
          .attr("fill", getNodeColor(d.type))
          .attr("stroke", "#ffffff")
          .attr("stroke-width", 2)
          .style("filter", "drop-shadow(0 2px 4px rgba(0,0,0,0.1))");
      } else if (d.type === "exchange") {
        // Exchange: Purple Diamond (same height as queue/consumer)
        node
          .append("rect")
          .attr("x", -15)
          .attr("y", -15)
          .attr("width", 30)
          .attr("height", 30)
          .attr("rx", 3)
          .attr("transform", "rotate(45)")
          .attr("fill", getNodeColor(d.type))
          .attr("stroke", "#ffffff")
          .attr("stroke-width", 2)
          .style("filter", "drop-shadow(0 2px 4px rgba(0,0,0,0.1))");
      } else if (d.type === "queue") {
        // Queue: Orange Rounded Rectangle
        node
          .append("rect")
          .attr("x", -50)
          .attr("y", -15)
          .attr("width", 100)
          .attr("height", 30)
          .attr("rx", 15)
          .attr("fill", getNodeColor(d.type))
          .attr("stroke", "#ffffff")
          .attr("stroke-width", 2)
          .style("filter", "drop-shadow(0 2px 4px rgba(0,0,0,0.1))");
      } else if (d.type === "consumer") {
        // Consumer: Green Circle (same height as queue)
        node
          .append("circle")
          .attr("r", 15)
          .attr("fill", getNodeColor(d.type))
          .attr("stroke", "#ffffff")
          .attr("stroke-width", 2)
          .style("filter", "drop-shadow(0 2px 4px rgba(0,0,0,0.1))");
      }
    });

    // Add labels below shapes
    nodeElements
      .append("text")
      .attr("text-anchor", "middle")
      .attr("dy", (d) => {
        // Position labels below the shapes
        if (d.type === "producer") return 40;
        if (d.type === "exchange") return 45;
        if (d.type === "queue") return 35;
        if (d.type === "consumer") return 40;
        return 35;
      })
      .attr("font-size", "14px")
      .attr("font-weight", "bold")
      .attr("fill", "#374151")
      .style("pointer-events", "none")
      .text((d) => d.label);
  }

  function simulateMessage() {
    messageCount++;

    const simulatedMessage = {
      id: Date.now(),
      demo: "fanout",
      data: {
        messageId: messageCount,
        content: `Message ${messageCount}`,
        timestamp: new Date().toISOString(),
      },
      timestamp: new Date().toLocaleTimeString(),
    };

    console.log("Simulating fanout message:", simulatedMessage);
    animateMessage(simulatedMessage);
  }

  function clearMessages() {
    messageCount = 0;
    activeMessages = [];
    queueToConsumerTransitions = [];

    // Clear any pending timeout
    if (queueToConsumerTimeout) {
      clearTimeout(queueToConsumerTimeout);
      queueToConsumerTimeout = null;
    }

    // Clear any active messages and highlights
    if (svg) {
      svg.select(".messages").selectAll("*").remove();
      svg.select(".connection-highlights").selectAll("*").remove();
    }
  }

  function animateMessage(message) {
    const messageId = `msg-${Date.now()}-${Math.random()}`;

    // Start with producer to exchange
    const firstConnection = currentDemo.connections[0];
    animateMessageSegment(messageId, firstConnection, true, message.data);
  }

  function animateMessageSegment(messageId, connection, messageType, data) {
    animateMessageSegmentWithHighlight(
      messageId,
      connection,
      messageType,
      data,
      null
    );
  }

  function animateMessageSegmentWithHighlight(
    messageId,
    connection,
    messageType,
    data,
    connectionId
  ) {
    const fromNode = currentDemo.nodes.find((n) => n.id === connection.from);
    const toNode = currentDemo.nodes.find((n) => n.id === connection.to);

    if (!fromNode || !toNode) return;

    // Highlight connection path with optional connection ID for fanout
    highlightConnection(connection, connectionId);

    // Continue with the animation
    animateMessageSegmentWithoutHighlight(
      messageId,
      connection,
      messageType,
      data
    );
  }

  function animateMessageSegmentWithoutHighlight(
    messageId,
    connection,
    messageType,
    data
  ) {
    const fromNode = currentDemo.nodes.find((n) => n.id === connection.from);
    const toNode = currentDemo.nodes.find((n) => n.id === connection.to);

    if (!fromNode || !toNode) return;

    // Create message circle with proper color based on phase
    const isOriginal = messageType === true || messageType === "original";
    const isFanoutFromExchange =
      fromNode.type === "exchange" && messageType === "fanout";
    const isFanoutToConsumer = messageType === "fanout-consumer";

    // Color logic:
    // - Original message (producer->exchange): RED
    // - Fanout from exchange (exchange->queues): RED (synchronized broadcast)
    // - Fanout to consumers (queue->consumer): GREEN (individual delivery)
    const messageColor =
      isOriginal || isFanoutFromExchange ? "#FF4444" : "#10B981";
    const gradientId =
      isOriginal || isFanoutFromExchange
        ? "messageGradient"
        : "fanoutMessageGradient";

    const message = svg
      .select(".messages")
      .append("g")
      .attr("class", "message-group")
      .attr("id", messageId);

    // Glow effect
    const glowColor =
      isOriginal || isFanoutFromExchange
        ? "rgba(255, 68, 68, 0.4)"
        : "rgba(16, 185, 129, 0.4)";
    message
      .append("circle")
      .attr("class", "message-glow")
      .attr("cx", fromNode.x)
      .attr("cy", fromNode.y)
      .attr("r", 8)
      .attr("fill", glowColor)
      .style("filter", "blur(2px)");

    // Main message
    const messageCircle = message
      .append("circle")
      .attr("class", "message-packet")
      .attr("cx", fromNode.x)
      .attr("cy", fromNode.y)
      .attr("r", 4)
      .attr("fill", `url(#${gradientId})`)
      .attr("stroke", messageColor)
      .attr("stroke-width", 1)
      .style(
        "filter",
        "drop-shadow(0 0 5px rgba(255, 68, 68, 1)) drop-shadow(0 0 10px rgba(255, 107, 53, 0.8))"
      );

    // Animate glow pulsing
    message
      .select(".message-glow")
      .transition()
      .duration(800)
      .attr("r", 12)
      .style("opacity", 0.2)
      .transition()
      .duration(800)
      .attr("r", 8)
      .style("opacity", 0.6)
      .on("end", function () {
        d3.select(this)
          .transition()
          .duration(600)
          .attr("r", 10)
          .style("opacity", 0.4);
      });

    // Animate message movement along curved path
    messageCircle
      .transition()
      .duration(animationSpeed)
      .ease(d3.easeCubicInOut)
      .tween("pathTween", function () {
        return function (t) {
          const pos = getPositionAlongCurve(fromNode, toNode, t);
          d3.select(this).attr("cx", pos.x).attr("cy", pos.y);

          // Update glow position to follow
          message.select(".message-glow").attr("cx", pos.x).attr("cy", pos.y);
        };
      })
      .on("end", function () {
        // Handle fanout at exchange
        if (
          toNode.type === "exchange" &&
          (messageType === true || messageType === "original")
        ) {
          setTimeout(() => {
            // Clear all existing highlights before starting fanout
            svg.select(".connection-highlights").selectAll("*").remove();

            createFanoutMessages(messageId, toNode.id, data);
            message.remove();
          }, 200);
        } else {
          // Continue to next segment if there is one
          const nextConnection = currentDemo.connections.find(
            (conn) => conn.from === toNode.id
          );
          if (nextConnection) {
            setTimeout(() => {
              message.remove();

              // Check if this is a queue-to-consumer transition that should be synchronized
              if (messageType === "fanout" && toNode.type === "queue") {
                // Store this transition for synchronized execution
                scheduleQueueToConsumerTransition(
                  messageId,
                  nextConnection,
                  data
                );
              } else {
                // Regular single message continuation
                animateMessageSegment(
                  `${messageId}-next`,
                  nextConnection,
                  messageType,
                  data
                );
              }
            }, 200);
          } else {
            // Remove message at final destination
            setTimeout(() => {
              message.remove();
            }, 500);
          }
        }
      });
  }

  function scheduleQueueToConsumerTransition(messageId, connection, data) {
    // Add this transition to the pending list
    queueToConsumerTransitions.push({
      messageId: `${messageId}-consumer`,
      connection: connection,
      data: data,
    });

    // Clear any existing timeout and set a new one
    // This ensures all queue-to-consumer transitions that arrive within a short window are batched
    if (queueToConsumerTimeout) {
      clearTimeout(queueToConsumerTimeout);
    }

    queueToConsumerTimeout = setTimeout(() => {
      executeQueueToConsumerTransitions();
    }, 50); // Small delay to batch all transitions
  }

  function executeQueueToConsumerTransitions() {
    if (queueToConsumerTransitions.length === 0) return;

    const transitions = [...queueToConsumerTransitions];
    queueToConsumerTransitions = []; // Clear the list
    queueToConsumerTimeout = null;

    // Create all highlights and start animations simultaneously
    transitions.forEach((transition, index) => {
      const connectionId = `queue-consumer-highlight-${Date.now()}-${index}`;

      // Create highlight
      highlightConnection(transition.connection, connectionId);

      // Start animation
      animateMessageSegmentWithoutHighlight(
        transition.messageId,
        transition.connection,
        "fanout-consumer",
        transition.data
      );
    });
  }

  function createFanoutMessages(originalMessageId, exchangeId, data) {
    const fanoutConnections = currentDemo.connections.filter(
      (conn) => conn.from === exchangeId
    );

    // All fanout messages start simultaneously at the exact same time
    const fanoutStartTime = Date.now();

    // Create all highlights and start animations simultaneously
    fanoutConnections.forEach((connection, index) => {
      const connectionId = `fanout-highlight-${fanoutStartTime}-${index}`;
      const fanoutMessageId = `fanout-${fanoutStartTime}-${index}`;

      // Create highlight
      highlightConnection(connection, connectionId);

      // Start fanout message animation
      animateMessageSegmentWithoutHighlight(
        fanoutMessageId,
        connection,
        "fanout",
        data
      );
    });
  }

  function highlightConnection(connection, connectionId = null) {
    const highlightGroup = svg.select(".connection-highlights");

    // For single connections, remove existing highlights
    // For fanout connections, keep existing highlights (don't clear)
    if (!connectionId) {
      highlightGroup.selectAll("*").remove();
    }

    // Create unique ID for this highlight
    const highlightId =
      connectionId || `highlight-${Date.now()}-${Math.random()}`;

    const pathData = getConnectionPath(connection.from, connection.to);

    // Add outer glow effect
    const outerGlow = highlightGroup
      .append("path")
      .attr("class", "connection-highlight-outer")
      .attr("id", `${highlightId}-outer`)
      .attr("d", pathData)
      .attr("stroke", "#FFA500")
      .attr("stroke-width", 6)
      .attr("fill", "none")
      .attr("stroke-linecap", "round")
      .style("opacity", 0.3);

    // Add middle glow effect
    const middleGlow = highlightGroup
      .append("path")
      .attr("class", "connection-highlight-middle")
      .attr("id", `${highlightId}-middle`)
      .attr("d", pathData)
      .attr("stroke", "#FF6B35")
      .attr("stroke-width", 4)
      .attr("fill", "none")
      .attr("stroke-linecap", "round")
      .style("opacity", 0.6);

    // Add core highlight
    const coreHighlight = highlightGroup
      .append("path")
      .attr("class", "connection-highlight-core")
      .attr("id", `${highlightId}-core`)
      .attr("d", pathData)
      .attr("stroke", "#FF4444")
      .attr("stroke-width", 2)
      .attr("fill", "none")
      .attr("stroke-linecap", "round")
      .style(
        "filter",
        "drop-shadow(0 0 4px rgba(255, 68, 68, 1)) drop-shadow(0 0 8px rgba(255, 107, 53, 0.8)) drop-shadow(0 0 12px rgba(255, 165, 0, 0.6))"
      )
      .attr("marker-end", "url(#arrowhead-active)");

    // Animate outer glow pulsing
    outerGlow
      .transition()
      .duration(1000)
      .style("opacity", 0.1)
      .transition()
      .duration(1000)
      .style("opacity", 0.4)
      .on("end", function repeat() {
        d3.select(this)
          .transition()
          .duration(1000)
          .style("opacity", 0.1)
          .transition()
          .duration(1000)
          .style("opacity", 0.4)
          .on("end", repeat);
      });

    // Animate middle glow pulsing
    middleGlow
      .transition()
      .duration(1200)
      .style("opacity", 0.4)
      .transition()
      .duration(1200)
      .style("opacity", 0.8)
      .on("end", function repeat() {
        d3.select(this)
          .transition()
          .duration(1200)
          .style("opacity", 0.4)
          .transition()
          .duration(1200)
          .style("opacity", 0.8)
          .on("end", repeat);
      });

    // Animate core color cycling
    coreHighlight
      .transition()
      .duration(2000)
      .attr("stroke", "#FF6B35")
      .transition()
      .duration(2000)
      .attr("stroke", "#FFA500")
      .transition()
      .duration(2000)
      .attr("stroke", "#FF6B35")
      .transition()
      .duration(2000)
      .attr("stroke", "#FF4444")
      .on("end", function repeat() {
        d3.select(this)
          .transition()
          .duration(2000)
          .attr("stroke", "#FF6B35")
          .transition()
          .duration(2000)
          .attr("stroke", "#FFA500")
          .transition()
          .duration(2000)
          .attr("stroke", "#FF6B35")
          .transition()
          .duration(2000)
          .attr("stroke", "#FF4444")
          .on("end", repeat);
      });

    // Remove all highlight layers after animation duration
    setTimeout(() => {
      highlightGroup.select(`#${highlightId}-outer`).remove();
      highlightGroup.select(`#${highlightId}-middle`).remove();
      highlightGroup.select(`#${highlightId}-core`).remove();
    }, animationSpeed);
  }

  function getNodeColor(type) {
    switch (type) {
      case "producer":
        return "#3B82F6";
      case "consumer":
        return "#10B981";
      case "queue":
        return "#F59E0B";
      case "exchange":
        return "#8B5CF6";
      default:
        return "#6B7280";
    }
  }

  function getConnectionPath(from, to) {
    const fromNode = currentDemo.nodes.find((n) => n.id === from);
    const toNode = currentDemo.nodes.find((n) => n.id === to);

    if (!fromNode || !toNode) return "";

    // Calculate node edge positions for better connection points
    let fromX, fromY, toX, toY;

    if (fromNode.type === "producer") {
      fromX = fromNode.x + 15; // Right edge of producer square
      fromY = fromNode.y;
    } else if (fromNode.type === "exchange") {
      fromX = fromNode.x + 21; // Right tip of exchange diamond (sqrt(30²/2) ≈ 21)
      fromY = fromNode.y;
    } else if (fromNode.type === "queue") {
      fromX = fromNode.x + 50; // Right edge of queue rectangle
      fromY = fromNode.y;
    } else {
      fromX = fromNode.x + 15; // Right edge of consumer circle
      fromY = fromNode.y;
    }

    if (toNode.type === "producer") {
      toX = toNode.x - 15; // Left edge of producer square
      toY = toNode.y;
    } else if (toNode.type === "exchange") {
      toX = toNode.x - 21; // Left tip of exchange diamond (sqrt(30²/2) ≈ 21)
      toY = toNode.y;
    } else if (toNode.type === "queue") {
      toX = toNode.x - 50; // Left edge of queue rectangle
      toY = toNode.y;
    } else {
      toX = toNode.x - 15; // Left edge of consumer circle
      toY = toNode.y;
    }

    // Create smooth curved path using cubic Bezier curves
    const midX = (fromX + toX) / 2;
    const controlPointOffset = Math.abs(toX - fromX) * 0.3; // 30% of horizontal distance

    // Control points for the curve
    const cp1X = fromX + controlPointOffset;
    const cp1Y = fromY;
    const cp2X = toX - controlPointOffset;
    const cp2Y = toY;

    return `M ${fromX} ${fromY} C ${cp1X} ${cp1Y}, ${cp2X} ${cp2Y}, ${toX} ${toY}`;
  }

  function getPositionAlongCurve(fromNode, toNode, progress) {
    // Calculate the same connection points as getConnectionPath
    let fromX, fromY, toX, toY;

    if (fromNode.type === "producer") {
      fromX = fromNode.x + 15;
      fromY = fromNode.y;
    } else if (fromNode.type === "exchange") {
      fromX = fromNode.x + 21;
      fromY = fromNode.y;
    } else if (fromNode.type === "queue") {
      fromX = fromNode.x + 50;
      fromY = fromNode.y;
    } else {
      fromX = fromNode.x + 15;
      fromY = fromNode.y;
    }

    if (toNode.type === "producer") {
      toX = toNode.x - 15;
      toY = toNode.y;
    } else if (toNode.type === "exchange") {
      toX = toNode.x - 21; // Left tip of exchange diamond (sqrt(30²/2) ≈ 21)
      toY = toNode.y;
    } else if (toNode.type === "queue") {
      toX = toNode.x - 50;
      toY = toNode.y;
    } else {
      toX = toNode.x - 15;
      toY = toNode.y;
    }

    // Same control points as getConnectionPath
    const controlPointOffset = Math.abs(toX - fromX) * 0.3;
    const cp1X = fromX + controlPointOffset;
    const cp1Y = fromY;
    const cp2X = toX - controlPointOffset;
    const cp2Y = toY;

    // Calculate position along cubic Bezier curve
    const t = progress;
    const oneMinusT = 1 - t;

    const x =
      Math.pow(oneMinusT, 3) * fromX +
      3 * Math.pow(oneMinusT, 2) * t * cp1X +
      3 * oneMinusT * Math.pow(t, 2) * cp2X +
      Math.pow(t, 3) * toX;

    const y =
      Math.pow(oneMinusT, 3) * fromY +
      3 * Math.pow(oneMinusT, 2) * t * cp1Y +
      3 * oneMinusT * Math.pow(t, 2) * cp2Y +
      Math.pow(t, 3) * toY;

    return { x, y };
  }

  function handleWebSocketMessage(event) {
    const message = event.detail;

    // Animate any incoming WebSocket message to show the fanout pattern
    console.log("Received WebSocket message:", message);

    const visualMessage = {
      id: Date.now(),
      demo: "fanout",
      data: message.data || message,
      timestamp: new Date().toLocaleTimeString(),
    };

    animateMessage(visualMessage);
  }
</script>

<div class="">
  <!-- Visualization -->
  <div class="bg-white border rounded-lg">
    <div class="border-b w-full p-3">
      <h3 class="text-lg font-semibold">{currentDemo.name}</h3>
      <p class="text-neutral-600">{currentDemo.description}</p>
    </div>
    <div class="flex items-center justify-between border-b p-3">
      <div class="flex items-center space-x-3">
        <button
          class="px-3 py-1.5 bg-green-700 text-white rounded-md hover:bg-green-600 transition-colors"
          on:click={simulateMessage}
        >
          Simulate Message
        </button>

        <button
          class="px-3 py-1.5 bg-gray-500 text-white rounded-md hover:bg-gray-600 transition-colors"
          on:click={clearMessages}
        >
          Clear
        </button>

        <div class="text-sm text-neutral-600">
          Messages Sent: {messageCount} | Connection: {connected
            ? "Connected"
            : "Disconnected"}
        </div>
      </div>

      <div class="flex items-center space-x-2">
        <label class="text-sm text-neutral-600">Speed:</label>
        <input
          type="range"
          min="400"
          max="1500"
          step="100"
          bind:value={animationSpeed}
          class="w-40"
        />
        <span class="text-sm text-neutral-600">{animationSpeed}ms</span>
      </div>
    </div>

    <div class="flex h-full">
      <!-- Chart Area -->
      <div class="flex-1 relative h-96 overflow-hidden border-r">
        <svg bind:this={svgElement} class="w-full h-full"></svg>
      </div>

      <!-- Legend -->
      <div class="space-y-6 h-full pr-3 pt-3 pb-3 overflow-y-auto">
        <!-- Producer -->
        <div class="flex items-center justify-start space-x-3">
          <svg width="48" height="24" viewBox="0 0 24 24" class="flex-shrink-0">
            <defs>
              <radialGradient
                id="legendMessageGradient"
                cx="50%"
                cy="50%"
                r="50%"
              >
                <stop offset="0%" style="stop-color:#FCA5A5;stop-opacity:1" />
                <stop offset="100%" style="stop-color:#EF4444;stop-opacity:1" />
              </radialGradient>
              <radialGradient
                id="legendFanoutMessageGradient"
                cx="50%"
                cy="50%"
                r="50%"
              >
                <stop offset="0%" style="stop-color:#A7F3D0;stop-opacity:1" />
                <stop offset="100%" style="stop-color:#10B981;stop-opacity:1" />
              </radialGradient>
            </defs>
            <rect
              x="2"
              y="2"
              width="20"
              height="20"
              rx="2"
              fill="#3B82F6"
              stroke="#ffffff"
              stroke-width="1"
              style="filter: drop-shadow(0 1px 2px rgba(0,0,0,0.1))"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-neutral-900">Producer</div>
            <div class="text-xs text-neutral-600">Sends messages</div>
          </div>
        </div>

        <!-- Exchange -->
        <div class="flex items-center justify-start space-x-3">
          <svg width="48" height="24" viewBox="0 0 24 24" class="flex-shrink-0">
            <rect
              x="2"
              y="2"
              width="20"
              height="20"
              rx="2"
              transform="rotate(45 12 12)"
              fill="#8B5CF6"
              stroke="#ffffff"
              stroke-width="1"
              style="filter: drop-shadow(0 1px 2px rgba(0,0,0,0.1))"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-neutral-900">Exchange</div>
            <div class="text-xs text-neutral-600">Routes messages</div>
          </div>
        </div>

        <!-- Queue -->
        <div class="flex items-center justify-start space-x-3">
          <svg width="48" height="24" viewBox="0 0 24 24" class="flex-shrink-0">
            <rect
              x="-13"
              y="2"
              width="40"
              height="20"
              rx="10"
              fill="#F59E0B"
              stroke="#ffffff"
              stroke-width="1"
              transform="translate(12,0)"
              style="filter: drop-shadow(0 1px 2px rgba(0,0,0,0.1))"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-neutral-900">Queue</div>
            <div class="text-xs text-neutral-600">Stores messages</div>
          </div>
        </div>

        <!-- Consumer -->
        <div class="flex items-center justify-start space-x-3">
          <svg width="48" height="24" viewBox="0 0 24 24" class="flex-shrink-0">
            <circle
              cx="12"
              cy="12"
              r="10"
              fill="#10B981"
              stroke="#ffffff"
              stroke-width="1"
              style="filter: drop-shadow(0 1px 2px rgba(0,0,0,0.1))"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-neutral-900">Consumer</div>
            <div class="text-xs text-neutral-600">Receives messages</div>
          </div>
        </div>

        <!-- Original Message -->
        <div class="flex items-center justify-start space-x-3">
          <svg width="48" height="24" viewBox="0 0 24 24" class="flex-shrink-0">
            <circle
              cx="12"
              cy="12"
              r="5"
              fill="url(#legendMessageGradient)"
              stroke="#FF4444"
              stroke-width="0.5"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-neutral-900">
              Original Message
            </div>
            <div class="text-xs text-neutral-600">Producer → Exchange</div>
          </div>
        </div>

        <!-- Fanout Message -->
        <div class="flex items-center justify-start space-x-3">
          <svg width="48" height="24" viewBox="0 0 24 24" class="flex-shrink-0">
            <circle
              cx="12"
              cy="12"
              r="5"
              fill="url(#legendFanoutMessageGradient)"
              stroke="#10B981"
              stroke-width="0.5"
            />
          </svg>
          <div>
            <div class="text-sm font-medium text-neutral-900">
              Fanout Message
            </div>
            <div class="text-xs text-neutral-600">Exchange → Consumers</div>
          </div>
        </div>
      </div>
    </div>
    <div class="p-3 border-t w-full">
      <h3 class="font-semibold mb-3">How Fanout Exchange Works</h3>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
        <div class="bg-blue-50 p-3 rounded">
          <h4 class="font-medium text-blue-900 mb-1">1. Producer Sends</h4>
          <p class="text-blue-700">
            Producer sends a single message to the fanout exchange
          </p>
        </div>
        <div class="bg-purple-50 p-3 rounded">
          <h4 class="font-medium text-purple-900 mb-1">
            2. Exchange Broadcasts
          </h4>
          <p class="text-purple-700">
            Fanout exchange duplicates the message to all bound queues
          </p>
        </div>
        <div class="bg-green-50 p-3 rounded">
          <h4 class="font-medium text-green-900 mb-1">3. Consumers Receive</h4>
          <p class="text-green-700">
            Each consumer gets a copy of the original message
          </p>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .message-packet {
    filter: drop-shadow(0 0 5px rgba(255, 68, 68, 1))
      drop-shadow(0 0 10px rgba(255, 107, 53, 0.8))
      drop-shadow(0 0 15px rgba(255, 165, 0, 0.6));
  }

  .message-glow {
    filter: blur(1px);
  }

  .active-path-core {
    filter: drop-shadow(0 0 4px rgba(255, 68, 68, 1))
      drop-shadow(0 0 8px rgba(255, 107, 53, 0.8))
      drop-shadow(0 0 12px rgba(255, 165, 0, 0.6));
  }

  svg {
    overflow: visible;
  }
</style>
