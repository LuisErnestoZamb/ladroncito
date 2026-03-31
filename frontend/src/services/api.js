export const submitAnalysis = async (data) => {

  const formData = new FormData();
  formData.append("initial_wallet", data.initial_wallet);
  formData.append("final_wallet", data.final_wallet);

  if (data.use_url) {
    formData.append("csv_url", data.csv_url);
    formData.append("use_url", true);
  } else if (data.csv_file && data.csv_file[0]) {
    formData.append("file", data.csv_file[0]);
  }

  const response = await fetch("/api/lavado", {
    method: "POST",
    body: formData,
  });

  if (!response.ok) {
    if (response.status === 429) throw new Error("Too many requests. Wait a minute.");
    throw new Error("Failed to start analysis");
  }

  return response.json();
};

export const listenToUpdates = (jobId,
  initial_wallet,
  final_wallet,
  onMessage) => {

  const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
  const socket = new WebSocket(`${protocol}//${window.location.host}/ws/analysis/${jobId}?initial_wallet=${encodeURIComponent(initial_wallet)}&final_wallet=${encodeURIComponent(final_wallet)}`);

  socket.onopen = () => console.log("WebSocket connected for job:", jobId);

  socket.onmessage = (event) => {
    onMessage(event.data);
  };

  socket.onerror = (error) => console.error("WebSocket Error:", error);

  socket.onclose = () => console.log("Analysis stream closed.");

  return socket;
};