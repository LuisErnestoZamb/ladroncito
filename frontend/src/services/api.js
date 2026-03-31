export const submitAnalysis = async (data) => {

  const formData = new FormData();
  formData.append("initialWallet", data.initialWallet);
  formData.append("finalWallet", data.finalWallet);

  if (data.useUrl) {
    formData.append("csvUrl", data.csvUrl);
  } else if (data.csvFile && data.csvFile[0]) {
    formData.append("file", data.csvFile[0]);
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
