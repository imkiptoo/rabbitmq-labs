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

  // Core features definitions
  const deadletterDemo = {
    name: "Dead Letter Exchange Pattern",
    description: "Failed messages are routed to a dead letter queue for analysis and recovery",
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
        label: "Main Exchange",
      },
      { id: "queue", type: "queue", x: 500, y: 200, label: "Processing Queue" },
      {
        id: "consumer",
        type: "consumer",
        x: 700,
        y: 200,
        label: "Consumer (Failing)",
      },
      {
        id: "dlx",
        type: "exchange",
        x: 500,
        y: 320,
        label: "Dead Letter Exchange",
      },
      { id: "dlq", type: "queue", x: 700, y: 320, label: "Dead Letter Queue" },
    ],
    connections: [
      { from: "producer", to: "exchange" },
      { from: "exchange", to: "queue" },
      { from: "queue", to: "consumer" },
      { from: "queue", to: "dlx" },
      { from: "dlx", to: "dlq" },
    ],
  };

  const ttlDemo = {
    name: "Message TTL Pattern",
    description: "Messages expire after a specified time if not consumed",
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
        label: "TTL Exchange",
      },
      { id: "queue", type: "queue", x: 500, y: 200, label: "TTL Queue (5s)" },
      {
        id: "consumer",
        type: "consumer",
        x: 700,
        y: 200,
        label: "Slow Consumer",
      },
      {
        id: "dlx",
        type: "exchange",
        x: 500,
        y: 320,
        label: "Expired Exchange",
      },
      { id: "expired_queue", type: "queue", x: 700, y: 320, label: "Expired Messages" },
    ],
    connections: [
      { from: "producer", to: "exchange" },
      { from: "exchange", to: "queue" },
      { from: "queue", to: "consumer" },
      { from: "queue", to: "dlx" },
      { from: "dlx", to: "expired_queue" },
    ],
  };

  const priorityDemo = {
    name: "Priority Queue Pattern",
    description: "Messages with higher priority are consumed before lower priority ones",
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
        label: "Priority Exchange",
      },
      { id: "priority_queue", type: "queue", x: 500, y: 200, label: "Priority Queue" },
      {
        id: "consumer1",
        type: "consumer",
        x: 700,
        y: 150,
        label: "Consumer 1",
      },
      {
        id: "consumer2",
        type: "consumer",
        x: 700,
        y: 250,
        label: "Consumer 2",
      },
    ],
    connections: [
      { from: "producer", to: "exchange" },
      { from: "exchange", to: "priority_queue" },
      { from: "priority_queue", to: "consumer1" },
      { from: "priority_queue", to: "consumer2" },
    ],
  };

  const clusteringDemo = {
    name: "Queue Mirroring Pattern",
    description: "Queues are replicated across multiple nodes for high availability",
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
        x: 250,
        y: 200,
        label: "HA Exchange",
      },
      { id: "queue_node1", type: "queue", x: 400, y: 150, label: "Queue (Node 1)" },
      { id: "queue_node2", type: "queue", x: 400, y: 200, label: "Queue (Node 2)" },
      { id: "queue_node3", type: "queue", x: 400, y: 250, label: "Queue (Node 3)" },
      {
        id: "consumer",
        type: "consumer",
        x: 600,
        y: 200,
        label: "Consumer",
      },
    ],
    connections: [
      { from: "producer", to: "exchange" },
      { from: "exchange", to: "queue_node1" },
      { from: "exchange", to: "queue_node2" },
      { from: "exchange", to: "queue_node3" },
      { from: "queue_node1", to: "consumer" },
      { from: "queue_node2", to: "consumer" },
      { from: "queue_node3", to: "consumer" },
    ],
  };

  const alternateDemo = {
    name: "Alternate Exchange Pattern",
    description: "Fallback routing when messages can't be routed to any queue",
    nodes: [
      {
        id: "producer",
        type: "producer",
        x: 100,
        y: 200,
        label: "Message Producer",
      },
      {
        id: "main_exchange",
        type: "exchange",
        x: 300,
        y: 200,
        label: "Main Exchange",
      },
      { id: "queue1", type: "queue", x: 500, y: 150, label: "Specific Queue" },
      {
        id: "consumer1",
        type: "consumer",
        x: 700,
        y: 150,
        label: "Specific Consumer",
      },
      {
        id: "alternate_exchange",
        type: "exchange",
        x: 500,
        y: 300,
        label: "Alternate Exchange",
      },
      { id: "fallback_queue", type: "queue", x: 700, y: 300, label: "Fallback Queue" },
    ],
    connections: [
      { from: "producer", to: "main_exchange" },
      { from: "main_exchange", to: "queue1" },
      { from: "queue1", to: "consumer1" },
      { from: "main_exchange", to: "alternate_exchange" },
      { from: "alternate_exchange", to: "fallback_queue" },
    ],
  };

  const demos = {
    deadletter: deadletterDemo,
    ttl: ttlDemo,
    priority: priorityDemo,
    clustering: clusteringDemo,
    alternate: alternateDemo,
  };

  let activeTab = "deadletter";
  let currentDemo = deadletterDemo;

  // Feature-specific state
  let ttlValue = 5000;
  let priorityLevel = 5;
  let messageLogs = [];
  let logIdCounter = 0;

  const featureTypes = {
    deadletter: {
      label: "Dead Letter Exchange",
      pattern: "failed-message",
      description: "Routes failed messages to DLX",
    },
    ttl: {
      label: "TTL Messages",
      pattern: "ttl-5s",
      description: "Messages expire after 5 seconds",
    },
    priority: {
      label: "Priority Messages",
      pattern: "priority-queue",
      description: "Higher priority messages processed first",
    },
    clustering: {
      label: "Clustered Queue",
      pattern: "ha-queue",
      description: "Queue mirrored across nodes",
    },
    alternate: {
      label: "Alternate Exchange",
      pattern: "fallback-routing",
      description: "Unroutable messages go to alternate",
    },
  };

  function addLogEntry(message, type = "info", data = null) {
    const timestamp = new Date().toLocaleTimeString();
    logIdCounter++;
    const logEntry = {
      message,
      type,
      timestamp,
      id: `log-${Date.now()}-${logIdCounter}`,
      data,
    };
    console.log("Adding log entry:", logEntry);
    messageLogs = [logEntry, ...messageLogs].slice(0, 50);
    console.log("Total logs now:", messageLogs.length);
  }

  function clearLogs() {
    messageLogs = [];
    logIdCounter = 0;
  }

  function switchTab(tabName) {
    activeTab = tabName;
    currentDemo = demos[tabName];

    clearMessages();
    clearLogs();

    if (svg) {
      svg.selectAll("*").interrupt();
      svg.selectAll("*").remove();
      initializeD3Visualization();
    }
  }

  onMount(() => {
    if (typeof window !== "undefined") {
      window.addEventListener("websocket-message", handleWebSocketMessage);
      initializeD3Visualization();

      addLogEntry(
        `📋 Core Features page loaded - Ready to simulate ${activeTab} feature`,
        "info"
      );

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

    // Create groups for different elements in proper z-order
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

  async function simulateMessage() {
    messageCount++;

    const selectedType = featureTypes[activeTab];
    let requestPayload = {
      feature: activeTab,
      pattern: selectedType.pattern,
      message: {
        messageId: messageCount,
        content: `${selectedType.label} message ${messageCount}`,
        timestamp: new Date().toISOString(),
        featureType: activeTab,
      },
    };

    // Add feature-specific data
    if (activeTab === "ttl") {
      requestPayload.message.ttl = ttlValue;
      requestPayload.message.expiresAt = new Date(Date.now() + ttlValue).toISOString();
    } else if (activeTab === "priority") {
      requestPayload.message.priority = priorityLevel;
    }

    console.log(`🚀 Sending ${activeTab} feature request:`, requestPayload);
    addLogEntry(`🚀 Sending request to ${selectedType.label}`, "info");
    addLogEntry(`📤 Request payload`, "json", requestPayload);

    // Create simulated message for animation
    const simulatedMessage = {
      id: Date.now(),
      demo: activeTab,
      data: requestPayload.message,
      timestamp: new Date().toLocaleTimeString(),
    };

    // Start animation immediately
    animateMessage(simulatedMessage);

    // Simulate feature behavior
    setTimeout(() => simulateFeatureBehavior(requestPayload), 1500);
  }

  function simulateFeatureBehavior(messageData) {
    switch (activeTab) {
      case "deadletter":
        const shouldFail = Math.random() > 0.6;
        if (shouldFail) {
          addLogEntry(`❌ Consumer rejected message`, "error");
          addLogEntry(`➡️ Message routed to Dead Letter Exchange`, "route");
          addLogEntry(`🏠 Message stored in Dead Letter Queue`, "success");
        } else {
          addLogEntry(`✅ Message processed successfully`, "success");
        }
        break;

      case "ttl":
        const willExpire = Math.random() > 0.5;
        if (willExpire) {
          addLogEntry(`⏰ Message expired after ${ttlValue}ms`, "warning");
          addLogEntry(`➡️ Expired message routed to TTL exchange`, "route");
        } else {
          addLogEntry(`✅ Message processed within TTL`, "success");
        }
        break;

      case "priority":
        const queuePosition = Math.max(1, 11 - priorityLevel);
        addLogEntry(`📊 Message priority: ${priorityLevel}/10`, "info");
        addLogEntry(`📍 Queue position: ${queuePosition}`, "route");
        addLogEntry(`⚡ Higher priority messages processed first`, "success");
        break;

      case "clustering":
        addLogEntry(`🔄 Message replicated across cluster nodes`, "info");
        addLogEntry(`📋 Synchronizing with mirror queues`, "route");
        addLogEntry(`✅ Message safely stored across multiple nodes`, "success");
        break;

      case "alternate":
        const useAlternate = Math.random() > 0.7;
        if (useAlternate) {
          addLogEntry(`❌ No queue bound for routing key`, "warning");
          addLogEntry(`➡️ Message routed to Alternate Exchange`, "route");
          addLogEntry(`📦 Message stored in fallback queue`, "info");
        } else {
          addLogEntry(`✅ Message routed to specific queue`, "success");
        }
        break;
    }
  }

  function clearMessages() {
    messageCount = 0;
    activeMessages = [];
    queueToConsumerTransitions = [];

    if (queueToConsumerTimeout) {
      clearTimeout(queueToConsumerTimeout);
      queueToConsumerTimeout = null;
    }

    if (svg) {
      svg.select(".messages").selectAll("*").remove();
      svg.select(".connection-highlights").selectAll("*").remove();
      svg.selectAll("*").interrupt();
    }
  }

  function animateMessage(message) {
    const messageId = `msg-${Date.now()}-${Math.floor(Math.random() * 10000)}`;

    if (currentDemo.connections.length > 0) {
      const firstConnection = currentDemo.connections[0];
      animateMessageSegment(messageId, firstConnection, true, message.data);
    }
  }

  function animateMessageSegment(messageId, connection, messageType, data) {
    const fromNode = currentDemo.nodes.find((n) => n.id === connection.from);
    const toNode = currentDemo.nodes.find((n) => n.id === connection.to);

    if (!fromNode || !toNode) return;

    highlightConnection(connection);

    const messageColor = "#FF4444";
    const message = svg
      .select(".messages")
      .append("g")
      .attr("class", "message-group")
      .attr("id", messageId);

    // Glow effect
    message
      .append("circle")
      .attr("class", "message-glow")
      .attr("cx", fromNode.x)
      .attr("cy", fromNode.y)
      .attr("r", 8)
      .attr("fill", "rgba(255, 68, 68, 0.4)")
      .style("filter", "blur(2px)");

    // Main message icon
    const messageIcon = message
      .append("g")
      .attr("class", "message-packet")
      .attr("transform", `translate(${fromNode.x - 8}, ${fromNode.y - 6})`);

    // Message envelope background
    messageIcon
      .append("rect")
      .attr("width", 16)
      .attr("height", 12)
      .attr("rx", 2)
      .attr("ry", 2)
      .attr("fill", messageColor)
      .attr("stroke", "white")
      .attr("stroke-width", 1)
      .style("filter", "drop-shadow(0 0 3px rgba(0, 0, 0, 0.3))");

    // Message content lines
    [4, 6, 8].forEach((y, i) => {
      messageIcon
        .append("line")
        .attr("x1", 3)
        .attr("y1", y)
        .attr("x2", y === 6 ? 10 : 13)
        .attr("y2", y)
        .attr("stroke", "white")
        .attr("stroke-width", 1)
        .attr("opacity", 0.8);
    });

    // Envelope fold line
    messageIcon
      .append("path")
      .attr("d", "M1 2 L8 7 L15 2")
      .attr("stroke", "white")
      .attr("stroke-width", 1)
      .attr("fill", "none")
      .attr("opacity", 0.7);

    // Animate message movement
    messageIcon
      .transition()
      .duration(animationSpeed)
      .ease(d3.easeCubicInOut)
      .tween("pathTween", function () {
        return function (t) {
          const pos = getPositionAlongCurve(fromNode, toNode, t);
          d3.select(this).attr(
            "transform",
            `translate(${pos.x - 8}, ${pos.y - 6})`
          );
          message.select(".message-glow").attr("cx", pos.x).attr("cy", pos.y);
        };
      })
      .on("end", function () {
        // Continue to next segment if available
        const nextConnection = currentDemo.connections.find(
          (conn) => conn.from === toNode.id
        );
        if (nextConnection) {
          setTimeout(() => {
            message.remove();
            animateMessageSegment(
              `${messageId}-next`,
              nextConnection,
              messageType,
              data
            );
          }, 200);
        } else {
          // Remove message at final destination
          setTimeout(() => {
            message.remove();
            svg.select(".connection-highlights").selectAll("*").remove();
          }, 500);
        }
      });
  }

  function highlightConnection(connection) {
    const highlightGroup = svg.select(".connection-highlights");
    highlightGroup.selectAll("*").remove();

    const pathData = getConnectionPath(connection.from, connection.to);

    const highlight = highlightGroup
      .append("path")
      .attr("d", pathData)
      .attr("stroke", "#FF4444")
      .attr("stroke-width", 4)
      .attr("fill", "none")
      .attr("stroke-linecap", "round")
      .style("opacity", 0.8)
      .attr("marker-end", "url(#arrowhead-active)");

    setTimeout(() => {
      highlight.remove();
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

    // Calculate node edge positions
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
      toX = toNode.x - 21;
      toY = toNode.y;
    } else if (toNode.type === "queue") {
      toX = toNode.x - 50;
      toY = toNode.y;
    } else {
      toX = toNode.x - 15;
      toY = toNode.y;
    }

    // Create smooth curved path
    const controlPointOffset = Math.abs(toX - fromX) * 0.3;
    const cp1X = fromX + controlPointOffset;
    const cp1Y = fromY;
    const cp2X = toX - controlPointOffset;
    const cp2Y = toY;

    return `M ${fromX} ${fromY} C ${cp1X} ${cp1Y}, ${cp2X} ${cp2Y}, ${toX} ${toY}`;
  }

  function getPositionAlongCurve(fromNode, toNode, progress) {
    // Calculate edge positions (same as getConnectionPath)
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
      toX = toNode.x - 21;
      toY = toNode.y;
    } else if (toNode.type === "queue") {
      toX = toNode.x - 50;
      toY = toNode.y;
    } else {
      toX = toNode.x - 15;
      toY = toNode.y;
    }

    // Control points
    const controlPointOffset = Math.abs(toX - fromX) * 0.3;
    const cp1X = fromX + controlPointOffset;
    const cp1Y = fromY;
    const cp2X = toX - controlPointOffset;
    const cp2Y = toY;

    // Cubic Bezier curve calculation
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
    console.log("Received WebSocket message:", message);

    const visualMessage = {
      id: Date.now(),
      demo: activeTab,
      data: message.data || message,
      timestamp: new Date().toLocaleTimeString(),
    };

    animateMessage(visualMessage);
  }
</script>

<div class="h-screen flex flex-col space-y-3">
  <!-- Tab Navigation -->
  <div class="bg-white border rounded-lg flex-shrink-0 overflow-hidden">
    <div class="border-b">
      <nav class="flex space-x-0" aria-label="Tabs">
        <button
          class="relative px-3 py-1 font-medium transition-colors duration-200 border-b-2 {activeTab ===
          'deadletter'
            ? 'text-red-600 border-red-600 bg-red-50'
            : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => switchTab("deadletter")}
        >
          Dead Letter Exchange
          <span
            class="absolute inset-x-0 bottom-0 h-0.5 bg-red-600 transition-all duration-200 {activeTab ===
            'deadletter'
              ? 'opacity-100'
              : 'opacity-0'}"
          ></span>
        </button>
        <button
          class="relative px-3 py-1 font-medium transition-colors duration-200 border-b-2 {activeTab ===
          'ttl'
            ? 'text-orange-600 border-orange-600 bg-orange-50'
            : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => switchTab("ttl")}
        >
          Message TTL
          <span
            class="absolute inset-x-0 bottom-0 h-0.5 bg-orange-600 transition-all duration-200 {activeTab ===
            'ttl'
              ? 'opacity-100'
              : 'opacity-0'}"
          ></span>
        </button>
        <button
          class="relative px-3 py-1 font-medium transition-colors duration-200 border-b-2 {activeTab ===
          'priority'
            ? 'text-purple-600 border-purple-600 bg-purple-50'
            : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => switchTab("priority")}
        >
          Priority Queues
          <span
            class="absolute inset-x-0 bottom-0 h-0.5 bg-purple-600 transition-all duration-200 {activeTab ===
            'priority'
              ? 'opacity-100'
              : 'opacity-0'}"
          ></span>
        </button>
        <button
          class="relative px-3 py-1 font-medium transition-colors duration-200 border-b-2 {activeTab ===
          'clustering'
            ? 'text-blue-600 border-blue-600 bg-blue-50'
            : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => switchTab("clustering")}
        >
          Queue Mirroring
          <span
            class="absolute inset-x-0 bottom-0 h-0.5 bg-blue-600 transition-all duration-200 {activeTab ===
            'clustering'
              ? 'opacity-100'
              : 'opacity-0'}"
          ></span>
        </button>
        <button
          class="relative px-3 py-1 font-medium transition-colors duration-200 border-b-2 {activeTab ===
          'alternate'
            ? 'text-green-600 border-green-600 bg-green-50'
            : 'text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => switchTab("alternate")}
        >
          Alternate Exchange
          <span
            class="absolute inset-x-0 bottom-0 h-0.5 bg-green-600 transition-all duration-200 {activeTab ===
            'alternate'
              ? 'opacity-100'
              : 'opacity-0'}"
          ></span>
        </button>
      </nav>
    </div>
    <div class="px-3 py-1">
      <div class="text-gray-600 flex flex-col space-y-1">
        <p>
          <strong>{featureTypes[activeTab].label}:</strong> {featureTypes[activeTab].description}
        </p>
        <div class="text-neutral-600">{currentDemo.description}</div>
      </div>
    </div>
  </div>

  <!-- Visualization -->
  <div class="bg-white border rounded-lg flex-1 flex flex-col">
    <div class="flex items-center justify-between border-b p-3 flex-shrink-0">
      <div class="flex items-center space-x-3">
        <button
          class="px-3 py-1 bg-green-700 text-white rounded-md hover:bg-green-600 transition-colors"
          on:click={simulateMessage}
        >
          Simulate Message
        </button>

        <button
          class="px-3 py-1 bg-gray-500 text-white rounded-md hover:bg-gray-600 transition-colors"
          on:click={clearMessages}
        >
          Clear
        </button>

        {#if activeTab === "ttl"}
          <div class="flex items-center space-x-2 border-l pl-3">
            <label class="font-medium text-neutral-700">TTL (ms):</label>
            <input
              type="number"
              bind:value={ttlValue}
              min="1000"
              max="30000"
              step="1000"
              class="w-20 px-2 py-1 border border-neutral-300 rounded"
            />
          </div>
        {:else if activeTab === "priority"}
          <div class="flex items-center space-x-2 border-l pl-3">
            <label class="font-medium text-neutral-700">Priority:</label>
            <input
              type="range"
              bind:value={priorityLevel}
              min="0"
              max="10"
              class="w-24"
            />
            <span class="text-sm text-neutral-600">{priorityLevel}</span>
          </div>
        {/if}

        <button
          class="px-3 py-1 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
          on:click={clearLogs}
        >
          Clear Logs
        </button>

        <div class="text text-neutral-600">
          Messages Sent: {messageCount} | Connection: {connected
            ? "Connected"
            : "Disconnected"}
        </div>
      </div>

      <div class="flex items-center space-x-2">
        <label class="text text-neutral-600">Speed:</label>
        <input
          type="range"
          min="400"
          max="1500"
          step="100"
          bind:value={animationSpeed}
          class="w-40"
        />
        <span class="text text-neutral-600">{animationSpeed}ms</span>
      </div>
    </div>

    <div class="flex flex-col h-full">
      <!-- Chart Area with Logs Panel -->
      <div class="flex-1 relative overflow-hidden">
        <div class="flex h-full">
          <!-- Visualization -->
          <div class="flex-1 relative overflow-hidden border-r max-h-96">
            <svg bind:this={svgElement} class="h-full"></svg>
          </div>

          <!-- Logs Panel -->
          <div class="w-80 bg-neutral-50 flex flex-col h-full">
            <div class="flex-1 overflow-y-auto p-3 space-y-2 min-h-0 max-h-96">
              {#if messageLogs.length > 0}
                {#each messageLogs as log (log.id)}
                  <div
                    class="text-xs p-2 rounded border-l-2 {log.type ===
                    'success'
                      ? 'border-green-400 bg-green-50'
                      : log.type === 'route'
                        ? 'border-blue-400 bg-blue-50'
                        : log.type === 'error'
                          ? 'border-red-400 bg-red-50'
                          : log.type === 'warning'
                            ? 'border-yellow-400 bg-yellow-50'
                            : log.type === 'json'
                              ? 'border-purple-400 bg-purple-50'
                              : 'border-neutral-400 bg-white'}"
                  >
                    <div class="flex justify-between items-start">
                      <div class="flex-1 pr-2">
                        <div class="text-neutral-800">{log.message}</div>
                        {#if log.data && log.type === "json"}
                          <div
                            class="mt-1 p-2 bg-neutral-100 rounded text-xs font-mono text-neutral-600 break-all"
                          >
                            {JSON.stringify(log.data, null, 2)}
                          </div>
                        {/if}
                      </div>
                      <div class="text-neutral-500 text-xs whitespace-nowrap">
                        {log.timestamp}
                      </div>
                    </div>
                  </div>
                {/each}
              {:else}
                <div class="text-center text-neutral-500 py-8">
                  No messages yet. Click "Simulate Message" to see {featureTypes[activeTab].label.toLowerCase()} behavior.
                </div>
              {/if}
            </div>
          </div>
        </div>
      </div>

      <div
        class="pt-3 pb-1 border-t overflow-y-auto items-center justify-center"
      >
        <h3 class="font-semibold px-3">Legend</h3>
        <!-- Legend -->
        <div
          class="grid grid-cols-6 px-3 pt-2 pb-2 overflow-y-auto items-center justify-center opacity-25 hover:opacity-100 transition-opacity"
        >
          <!-- Producer -->
          <div class="flex items-center justify-start space-x-3">
            <svg
              viewBox="0 0 24 24"
              class="flex-shrink-0 w-8 h-8 items-center justify-center"
            >
              <rect
                rx="3"
                class="fill-blue-500 stroke-white h-4 w-4 transform drop-shadow-md stroke-1"
              />
            </svg>
            <div>
              <div class="text-sm font-medium text-neutral-900">Producer</div>
              <div class="text-xs text-neutral-600">Sends messages</div>
            </div>
          </div>

          <!-- Exchange -->
          <div class="flex items-center justify-start space-x-3">
            <svg viewBox="0 0 24 24" class="flex-shrink-0 w-8 h-8">
              <rect
                rx="3"
                class="fill-purple-600 stroke-white h-4 w-4 transform rotate-45 drop-shadow-md stroke-1"
              />
            </svg>
            <div>
              <div class="text-sm font-medium text-neutral-900">Exchange</div>
              <div class="text-xs text-neutral-600">Routes messages</div>
            </div>
          </div>

          <!-- Queue -->
          <div class="flex items-center justify-start space-x-3">
            <svg
              viewBox="0 0 24 24"
              class="flex-shrink-0 w-12 h-7 rounded-full"
            >
              <rect
                rx="8"
                class="fill-amber-500 stroke-white h-4 w-8 transform drop-shadow-md stroke-1 rounded-full"
              />
            </svg>
            <div>
              <div class="text-sm font-medium text-neutral-900">Queue</div>
              <div class="text-xs text-neutral-600">Stores messages</div>
            </div>
          </div>

          <!-- Consumer -->
          <div class="flex items-center justify-start space-x-3">
            <svg
              viewBox="0 0 24 24"
              class="flex-shrink-0 w-8 h-8 items-center justify-center rounded-full"
            >
              <rect
                rx="30"
                class="fill-emerald-500 stroke-white h-4 w-4 transform drop-shadow-md rounded-full stroke-1"
              />
            </svg>
            <div>
              <div class="text-sm font-medium text-neutral-900">Consumer</div>
              <div class="text-xs text-neutral-600">Receives messages</div>
            </div>
          </div>

          <!-- Original Message -->
          <div class="flex items-center justify-start space-x-3">
            <svg
              width="48"
              height="24"
              viewBox="0 0 24 24"
              class="flex-shrink-0"
            >
              <g>
                <rect
                  width="16"
                  height="12"
                  rx="2"
                  ry="2"
                  fill="#FF4444"
                  stroke="white"
                  stroke-width="1"
                  style="filter: drop-shadow(0 0 3px rgba(0, 0, 0, 0.3))"
                />
                <line
                  x1="3"
                  y1="4"
                  x2="13"
                  y2="4"
                  stroke="white"
                  stroke-width="1"
                  opacity="0.8"
                />
                <line
                  x1="3"
                  y1="6"
                  x2="10"
                  y2="6"
                  stroke="white"
                  stroke-width="1"
                  opacity="0.6"
                />
                <line
                  x1="3"
                  y1="8"
                  x2="12"
                  y2="8"
                  stroke="white"
                  stroke-width="1"
                  opacity="0.8"
                />
                <path
                  d="M1 2 L8 7 L15 2"
                  stroke="white"
                  stroke-width="1"
                  fill="none"
                  opacity="0.7"
                />
              </g>
            </svg>
            <div>
              <div class="text-sm font-medium text-neutral-900">
                Core Message
              </div>
              <div class="text-xs text-neutral-600">Feature demonstration</div>
            </div>
          </div>

          <!-- Dead Letter -->
          <div class="flex items-center justify-start space-x-3">
            <svg
              width="48"
              height="24"
              viewBox="0 0 24 24"
              class="flex-shrink-0"
            >
              <g>
                <rect
                  width="16"
                  height="12"
                  rx="2"
                  ry="2"
                  fill="#DC2626"
                  stroke="white"
                  stroke-width="1"
                  style="filter: drop-shadow(0 0 3px rgba(0, 0, 0, 0.3))"
                />
                <text
                  x="8"
                  y="8"
                  text-anchor="middle"
                  font-size="8px"
                  font-weight="bold"
                  fill="white"
                >✗</text>
              </g>
            </svg>
            <div>
              <div class="text-sm font-medium text-neutral-900">
                Failed Message
              </div>
              <div class="text-xs text-neutral-600">Dead letter/TTL expired</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="p-3 border-t w-full">
      <h3 class="font-semibold mb-3">
        How {featureTypes[activeTab].label} Works
      </h3>

      {#if activeTab === "deadletter"}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div class="bg-blue-50 p-3 rounded">
            <h4 class="font-medium text-blue-900 mb-1">1. Message Processing</h4>
            <p class="text-blue-700">
              Consumer attempts to process message but fails or rejects it
            </p>
          </div>
          <div class="bg-red-50 p-3 rounded">
            <h4 class="font-medium text-red-900 mb-1">2. Dead Letter Routing</h4>
            <p class="text-red-700">
              Failed message is automatically routed to Dead Letter Exchange
            </p>
          </div>
          <div class="bg-yellow-50 p-3 rounded">
            <h4 class="font-medium text-yellow-900 mb-1">3. Analysis & Recovery</h4>
            <p class="text-yellow-700">
              Dead letters can be analyzed, fixed, and reprocessed
            </p>
          </div>
        </div>
      {:else if activeTab === "ttl"}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div class="bg-orange-50 p-3 rounded">
            <h4 class="font-medium text-orange-900 mb-1">1. TTL Configuration</h4>
            <p class="text-orange-700">
              Messages are sent with Time-To-Live value in milliseconds
            </p>
          </div>
          <div class="bg-yellow-50 p-3 rounded">
            <h4 class="font-medium text-yellow-900 mb-1">2. Expiration Timer</h4>
            <p class="text-yellow-700">
              Messages expire if not consumed within TTL period
            </p>
          </div>
          <div class="bg-red-50 p-3 rounded">
            <h4 class="font-medium text-red-900 mb-1">3. Expired Handling</h4>
            <p class="text-red-700">
              Expired messages are routed to dead letter or dropped
            </p>
          </div>
        </div>
      {:else if activeTab === "priority"}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div class="bg-purple-50 p-3 rounded">
            <h4 class="font-medium text-purple-900 mb-1">1. Priority Assignment</h4>
            <p class="text-purple-700">
              Messages are sent with priority values (0-255, higher = more priority)
            </p>
          </div>
          <div class="bg-indigo-50 p-3 rounded">
            <h4 class="font-medium text-indigo-900 mb-1">2. Queue Ordering</h4>
            <p class="text-indigo-700">
              Queue maintains messages in priority order, not FIFO
            </p>
          </div>
          <div class="bg-blue-50 p-3 rounded">
            <h4 class="font-medium text-blue-900 mb-1">3. Priority Consumption</h4>
            <p class="text-blue-700">
              Consumers receive highest priority messages first
            </p>
          </div>
        </div>
      {:else if activeTab === "clustering"}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div class="bg-blue-50 p-3 rounded">
            <h4 class="font-medium text-blue-900 mb-1">1. Queue Mirroring</h4>
            <p class="text-blue-700">
              Queue contents are replicated across multiple cluster nodes
            </p>
          </div>
          <div class="bg-green-50 p-3 rounded">
            <h4 class="font-medium text-green-900 mb-1">2. High Availability</h4>
            <p class="text-green-700">
              If one node fails, other nodes can continue serving the queue
            </p>
          </div>
          <div class="bg-purple-50 p-3 rounded">
            <h4 class="font-medium text-purple-900 mb-1">3. Synchronization</h4>
            <p class="text-purple-700">
              All mirror nodes stay synchronized for consistent data
            </p>
          </div>
        </div>
      {:else if activeTab === "alternate"}
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div class="bg-red-50 p-3 rounded">
            <h4 class="font-medium text-red-900 mb-1">1. Routing Failure</h4>
            <p class="text-red-700">
              Message cannot be routed to any bound queue
            </p>
          </div>
          <div class="bg-orange-50 p-3 rounded">
            <h4 class="font-medium text-orange-900 mb-1">2. Alternate Routing</h4>
            <p class="text-orange-700">
              Message is automatically sent to configured alternate exchange
            </p>
          </div>
          <div class="bg-yellow-50 p-3 rounded">
            <h4 class="font-medium text-yellow-900 mb-1">3. Fallback Processing</h4>
            <p class="text-yellow-700">
              Alternate exchange handles unroutable messages
            </p>
          </div>
        </div>
      {/if}
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

  svg {
    overflow: visible;
  }
</style>