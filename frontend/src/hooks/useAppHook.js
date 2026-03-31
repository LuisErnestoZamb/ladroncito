import { useForm } from "react-hook-form";
import { useState } from "preact/hooks";
import { listenToUpdates, submitAnalysis } from "../services/api";

export const useAppHook = () => {
  const { register, handleSubmit, watch, setValue, formState: { errors } } = useForm({
    defaultValues: {
      use_url: false,
      initial_wallet: '',
      final_wallet: '',
      csv_url: '',
      csv_file: null
    }
  });

  const isUrlMode = watch("use_url");

  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [results, setResults] = useState([]);

  const onSubmit = async (data) => {
    setIsAnalyzing(true);
    setResults([]);

    try {
      const { job_id } = await submitAnalysis(data);

      listenToUpdates(job_id,
        data.initial_wallet,
        data.final_wallet,
        (update) => {
          setResults((prev) => [...prev, update]);

          if (update.status === "completed") {
            setIsAnalyzing(false);
          }
        });

    } catch (error) {
      alert(error.message);
      setIsAnalyzing(false);
    }
  };

  return {
    isUrlMode, register, handleSubmit, setValue, errors, onSubmit, results
  }
};
