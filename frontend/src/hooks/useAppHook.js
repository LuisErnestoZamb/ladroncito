import { useForm } from "react-hook-form";
import { useState } from "preact/hooks";

export const useAppHook = () => {
  const { register, handleSubmit, watch, setValue, formState: { errors } } = useForm({
    defaultValues: {
      useUrl: false,
      initialWallet: '',
      finalWallet: '',
      csvUrl: '',
      csvFile: null
    }
  });

  const isUrlMode = watch("useUrl");

  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [results, setResults] = useState([]);

  const onSubmit = async (data) => {
    setIsAnalyzing(true);
    setResults([]);

    try {
      const { job_id } = await submitAnalysis(data);

    } catch (error) {
      alert(error.message);
      setIsAnalyzing(false);
    }
  };

  return {
    isUrlMode, register, handleSubmit, setValue, errors, onSubmit,
  }
};
