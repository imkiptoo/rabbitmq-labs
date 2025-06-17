<script>
  import { onMount, createEventDispatcher } from "svelte";
  import * as d3 from "d3";

  export let demo;
  export let animationSpeed = 800;
  export let activeMessages = [];

  const dispatch = createEventDispatcher();

  let svgElement;
  let svg;
  let queueToConsumerTransitions = [];
  let queueToConsumerTimeout = null;

  onMount(() => {
    initializeD3Visualization();
  });

  $: if (demo && svg) {
    updateVisualization();
  }

  function initializeD3Visualization() {
    const width = 900;
    const height = 400;

    svg = d3
      .select(svgElement)
      .attr("width", width)
      .attr("height", height)
      .attr("viewBox", `0 0 ${width} ${height}`);

    createGradientDefinitions();
    createArrowMarkers();
    createElementGroups();
  }

  function createGradientDefinitions() {
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

    // TTL gradient (orange/red for expiring messages)
    const ttlGradient = defs
      .append("radialGradient")
      .attr("id", "ttlMessageGradient")
      .attr("cx", "50%")
      .attr("cy", "50%")
      .attr("r", "50%");
    ttlGradient
      .append("stop")
      .attr("offset", "0%")
      .attr("stop-color", "#FED7AA")
      .attr("stop-opacity", 1);
    ttlGradient
      .append("stop")
      .attr("offset", "100%")
      .attr("stop-color", "#EA580C")
      .attr("stop-opacity", 1);

    // Priority gradient (purple for high priority)
    const priorityGradient = defs
      .append("radialGradient")
      .attr("id", "priorityMessageGradient")
      .attr("cx", "50%")
      .attr("cy", "50%")
      .attr("r", "50%");
    priorityGradient
      .append("stop")
      .attr("offset", "0%")
      .attr("stop-color", "#DDD6FE")
      .attr("stop-opacity", 1);
    priorityGradient
      .append("stop")
      .attr("offset", "100%")
      .attr("stop-color", "#7C3AED")
      .attr("stop-opacity", 1);
  }

  function createArrowMarkers() {
    const defs = svg.select("defs");

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

    defs
      .append("marker")
      .attr("id", "arrowhead-active-green")
      .attr("markerWidth", 10)
      .attr("markerHeight", 7)
      .attr("refX", 5)
      .attr("refY", 3.5)
      .attr("orient", "auto")
      .append("polygon")
      .attr("points", "0 0, 5 3.5, 0 7")
      .attr("fill", "#10B981");

    // Dead letter arrow (red/black)
    defs
      .append("marker")
      .attr("id", "arrowhead-deadletter")
      .attr("markerWidth", 10)
      .attr("markerHeight", 7)
      .attr("refX", 5)
      .attr("refY", 3.5)
      .attr("orient", "auto")
      .append("polygon")
      .attr("points", "0 0, 5 3.5, 0 7")
      .attr("fill", "#DC2626");
  }

  function createElementGroups() {
    svg.append("g").attr("class", "connections");
    svg.append("g").attr("class", "nodes");
    svg.append("g").attr("class", "connection-highlights");
    svg.append("g").attr("class", "messages");
    svg.append("g").attr("class", "annotations");
  }

  function updateVisualization() {
    // Clear existing elements
    svg.selectAll("*").interrupt();
    svg.select(".connections").selectAll("*").remove();
    svg.select(".nodes").selectAll("*").remove();
    svg.select(".connection-highlights").selectAll("*").remove();
    svg.select(".messages").selectAll("*").remove();
    svg.select(".annotations").selectAll("*").remove();

    if (demo) {
      drawConnections();
      drawNodes();
      drawAnnotations();
    }
  }

  function drawConnections() {
    const connectionsGroup = svg.select(".connections");

    connectionsGroup
      .selectAll(".connection")
      .data(demo.connections)
      .enter()
      .append("path")
      .attr("class", "connection")
      .attr("d", (d) => getConnectionPath(d.from, d.to))
      .attr("stroke", (d) => getConnectionColor(d))
      .attr("stroke-width", (d) => getConnectionWidth(d))
      .attr("fill", "none")
      .attr("stroke-dasharray", (d) => getConnectionDashArray(d))
      .attr("marker-end", (d) => getConnectionMarker(d));
  }

  function drawNodes() {
    const nodesGroup = svg.select(".nodes");

    const nodeElements = nodesGroup
      .selectAll(".node")
      .data(demo.nodes)
      .enter()
      .append("g")
      .attr("class", "node")
      .attr("transform", (d) => `translate(${d.x}, ${d.y})`);

    // Draw node shapes based on type
    nodeElements.each(function (d) {
      const node = d3.select(this);
      drawNodeShape(node, d);
    });

    // Add labels
    nodeElements
      .append("text")
      .attr("text-anchor", "middle")
      .attr("dy", (d) => getLabelOffset(d))
      .attr("font-size", "14px")
      .attr("font-weight", "bold")
      .attr("fill", "#374151")
      .style("pointer-events", "none")
      .text((d) => d.label);

    // Add node status indicators
    nodeElements
      .filter((d) => d.status)
      .append("circle")
      .attr("cx", 20)
      .attr("cy", -20)
      .attr("r", 4)
      .attr("fill", (d) => getStatusColor(d.status))
      .attr("stroke", "white")
      .attr("stroke-width", 1);
  }

  function drawNodeShape(node, d) {
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
    } else if (d.type === "queue" || d.type === "dlq") {
      const color = d.type === "dlq" ? "#DC2626" : getNodeColor(d.type);
      node
        .append("rect")
        .attr("x", -50)
        .attr("y", -15)
        .attr("width", 100)
        .attr("height", 30)
        .attr("rx", 15)
        .attr("fill", color)
        .attr("stroke", "#ffffff")
        .attr("stroke-width", 2)
        .style("filter", "drop-shadow(0 2px 4px rgba(0,0,0,0.1))");
      
      // Add queue depth indicator
      if (d.depth !== undefined) {
        node
          .append("text")
          .attr("text-anchor", "middle")
          .attr("dy", 4)
          .attr("font-size", "12px")
          .attr("font-weight", "bold")
          .attr("fill", "white")
          .text(d.depth);
      }
    } else if (d.type === "consumer") {
      node
        .append("circle")
        .attr("r", 15)
        .attr("fill", getNodeColor(d.type))
        .attr("stroke", "#ffffff")
        .attr("stroke-width", 2)
        .style("filter", "drop-shadow(0 2px 4px rgba(0,0,0,0.1))");
    }
  }

  function drawAnnotations() {
    const annotationsGroup = svg.select(".annotations");
    
    if (demo.annotations) {
      demo.annotations.forEach((annotation) => {
        const group = annotationsGroup
          .append("g")
          .attr("transform", `translate(${annotation.x}, ${annotation.y})`);

        group
          .append("rect")
          .attr("x", -40)
          .attr("y", -10)
          .attr("width", 80)
          .attr("height", 20)
          .attr("rx", 10)
          .attr("fill", annotation.color || "#FEF3C7")
          .attr("stroke", "#F59E0B")
          .attr("stroke-width", 1);

        group
          .append("text")
          .attr("text-anchor", "middle")
          .attr("dy", 4)
          .attr("font-size", "10px")
          .attr("font-weight", "bold")
          .attr("fill", "#92400E")
          .text(annotation.text);
      });
    }
  }

  function getNodeColor(type) {
    switch (type) {
      case "producer":
        return "#3B82F6";
      case "consumer":
        return "#10B981";
      case "queue":
        return "#F59E0B";
      case "dlq":
        return "#DC2626";
      case "exchange":
        return "#8B5CF6";
      default:
        return "#6B7280";
    }
  }

  function getConnectionColor(connection) {
    if (connection.type === "deadletter") return "#DC2626";
    if (connection.type === "priority") return "#7C3AED";
    return "#d0d0d0";
  }

  function getConnectionWidth(connection) {
    if (connection.type === "priority") return 3;
    return 2;
  }

  function getConnectionDashArray(connection) {
    if (connection.type === "deadletter") return "5,5";
    if (connection.type === "ttl") return "3,3";
    return "none";
  }

  function getConnectionMarker(connection) {
    if (connection.type === "deadletter") return "url(#arrowhead-deadletter)";
    return "url(#arrowhead)";
  }

  function getStatusColor(status) {
    switch (status) {
      case "active": return "#10B981";
      case "error": return "#DC2626";
      case "warning": return "#F59E0B";
      default: return "#6B7280";
    }
  }

  function getLabelOffset(d) {
    if (d.type === "producer") return 40;
    if (d.type === "exchange") return 45;
    if (d.type === "queue" || d.type === "dlq") return 35;
    if (d.type === "consumer") return 40;
    return 35;
  }

  function getConnectionPath(from, to) {
    const fromNode = demo.nodes.find((n) => n.id === from);
    const toNode = demo.nodes.find((n) => n.id === to);

    if (!fromNode || !toNode) return "";

    const { fromX, fromY } = getNodeEdgePosition(fromNode, true);
    const { toX, toY } = getNodeEdgePosition(toNode, false);

    const controlPointOffset = Math.abs(toX - fromX) * 0.3;
    const cp1X = fromX + controlPointOffset;
    const cp1Y = fromY;
    const cp2X = toX - controlPointOffset;
    const cp2Y = toY;

    return `M ${fromX} ${fromY} C ${cp1X} ${cp1Y}, ${cp2X} ${cp2Y}, ${toX} ${toY}`;
  }

  function getNodeEdgePosition(node, isSource) {
    let x, y;
    
    if (node.type === "producer") {
      x = node.x + (isSource ? 15 : -15);
      y = node.y;
    } else if (node.type === "exchange") {
      x = node.x + (isSource ? 21 : -21);
      y = node.y;
    } else if (node.type === "queue" || node.type === "dlq") {
      x = node.x + (isSource ? 50 : -50);
      y = node.y;
    } else {
      x = node.x + (isSource ? 15 : -15);
      y = node.y;
    }

    return { fromX: x, fromY: y, toX: x, toY: y };
  }

  // Animation functions
  export function animateMessage(messageData) {
    const messageId = `msg-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
    
    if (demo.connections.length > 0) {
      animateMessageSegment(messageId, demo.connections[0], messageData);
    }

    dispatch('messageAnimated', { messageId, data: messageData });
  }

  function animateMessageSegment(messageId, connection, data) {
    const fromNode = demo.nodes.find((n) => n.id === connection.from);
    const toNode = demo.nodes.find((n) => n.id === connection.to);

    if (!fromNode || !toNode) return;

    // Create message with appropriate styling based on message type
    const messageColor = getMessageColor(data);
    const gradientId = getMessageGradient(data);

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
      .attr("fill", `${messageColor}40`)
      .style("filter", "blur(2px)");

    // Main message icon
    const messageIcon = message
      .append("g")
      .attr("class", "message-packet")
      .attr("transform", `translate(${fromNode.x - 8}, ${fromNode.y - 6})`);

    drawMessageIcon(messageIcon, messageColor, data);

    // Animate movement
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
        // Handle message arrival
        handleMessageArrival(messageId, toNode, data, message);
      });
  }

  function drawMessageIcon(messageIcon, color, data) {
    const messageRect = messageIcon
      .append("rect")
      .attr("width", 16)
      .attr("height", 12)
      .attr("rx", 2)
      .attr("ry", 2)
      .attr("fill", color)
      .attr("stroke", "white")
      .attr("stroke-width", 1)
      .style("filter", "drop-shadow(0 0 3px rgba(0, 0, 0, 0.3))");

    // Add message type indicators
    if (data?.type === "priority" && data?.priority === "high") {
      messageIcon
        .append("text")
        .attr("x", 8)
        .attr("y", 8)
        .attr("text-anchor", "middle")
        .attr("font-size", "8px")
        .attr("font-weight", "bold")
        .attr("fill", "white")
        .text("!");
    } else if (data?.type === "ttl") {
      messageIcon
        .append("text")
        .attr("x", 8)
        .attr("y", 8)
        .attr("text-anchor", "middle")
        .attr("font-size", "8px")
        .attr("font-weight", "bold")
        .attr("fill", "white")
        .text("⏱");
    }

    // Standard message lines
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
  }

  function getMessageColor(data) {
    if (data?.type === "deadletter") return "#DC2626";
    if (data?.type === "ttl") return "#EA580C";
    if (data?.type === "priority" && data?.priority === "high") return "#7C3AED";
    return "#FF4444";
  }

  function getMessageGradient(data) {
    if (data?.type === "ttl") return "ttlMessageGradient";
    if (data?.type === "priority") return "priorityMessageGradient";
    return "messageGradient";
  }

  function handleMessageArrival(messageId, toNode, data, message) {
    // Dispatch event for message handling
    dispatch('messageArrived', { 
      messageId, 
      nodeId: toNode.id, 
      nodeType: toNode.type, 
      data 
    });

    // Default cleanup
    setTimeout(() => {
      message.remove();
    }, 500);
  }

  function getPositionAlongCurve(fromNode, toNode, progress) {
    const { fromX, fromY } = getNodeEdgePosition(fromNode, true);
    const { toX, toY } = getNodeEdgePosition(toNode, false);

    const controlPointOffset = Math.abs(toX - fromX) * 0.3;
    const cp1X = fromX + controlPointOffset;
    const cp1Y = fromY;
    const cp2X = toX - controlPointOffset;
    const cp2Y = toY;

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

  export function clearMessages() {
    if (svg) {
      svg.select(".messages").selectAll("*").remove();
      svg.select(".connection-highlights").selectAll("*").remove();
      svg.selectAll("*").interrupt();
    }
  }

  export function highlightConnection(connection, duration = 2000) {
    const highlightGroup = svg.select(".connection-highlights");
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
    }, duration);
  }
</script>

<svg bind:this={svgElement} class="w-full h-full"></svg>

<style>
  svg {
    overflow: visible;
  }
</style>