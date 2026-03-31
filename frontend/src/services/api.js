export const submitAnalysis = async (data) => {

  const formData = new FormData();
  formData.append("initial_wallet", data.initial_wallet);
  formData.append("final_wallet", data.final_wallet);

  if (data.use_url) {
    formData.append("csv_url", data.csv_url);
  } else if (data.csv_file && data.csv_file[0]) {
    formData.append("file", data.csv_file[0]);
  }

  const response = await fetch("http://127.0.0.1:8000/api/lavado", {
    method: "POST",
    body: formData,
  });

  if (!response.ok) {
    if (response.status === 429) throw new Error("Too many requests. Wait a minute.");
    throw new Error("Failed to start analysis");
  }

  return response.json();
};
