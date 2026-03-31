import { useAppHook } from "./hooks/useAppHook";

export function App() {
  const {
    isUrlMode, register, handleSubmit, setValue, errors, onSubmit,
  } = useAppHook();

  return (
    <div className="min-h-screen bg-slate-50 py-12 px-4">
      <div className="max-w-2xl mx-auto bg-white shadow-xl rounded-2xl p-8 border border-slate-200">

        <header className="text-center mb-8">
          <h1 className="text-2xl font-bold text-slate-900">USDT Route Finder</h1>
          <p className="text-sm text-slate-500 mt-2">
            Find transaction routes between two wallets using a CSV dataset.
          </p>
        </header>

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">

          {/* Wallets */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-semibold text-slate-700 mb-1">Initial Wallet</label>
              <input
                {...register("initialWallet", {
                  required: "Required",
                  pattern: { value: /^T[1-9A-HJ-NP-Za-km-z]{33}$/, message: "Invalid Tron address" }
                })}
                placeholder="TAs9YsYy..."
                className={`w-full p-2.5 border rounded-lg font-mono text-sm outline-none transition-all ${errors.initialWallet ? 'border-red-500 bg-red-50' : 'border-slate-300 focus:ring-2 focus:ring-blue-500'}`}
              />
              {errors.initialWallet && <p className="text-[10px] text-red-500 mt-1 uppercase font-bold">{errors.initialWallet.message}</p>}
            </div>

            <div>
              <label className="block text-sm font-semibold text-slate-700 mb-1">Final Wallet</label>
              <input
                {...register("finalWallet", { required: "Required" })}
                placeholder="TV6MuMXf..."
                className="w-full p-2.5 border border-slate-300 rounded-lg font-mono text-sm focus:ring-2 focus:ring-blue-500 outline-none"
              />
            </div>
          </div>

          <hr className="border-slate-100" />

          <div>
            <div className="flex items-center justify-between mb-4">
              <label className="text-sm font-semibold text-slate-700">Transaction Data Source</label>
              <button
                type="button"
                onClick={() => setValue("useUrl", !isUrlMode)} // Cambia el valor interno de React Hook Form
                className="text-xs text-blue-600 hover:underline font-medium bg-blue-50 px-2 py-1 rounded"
              >
                {isUrlMode ? "Switch to File Upload" : "Switch to External URL"}
              </button>
            </div>

            {/* Input vs Attachment */}
            {isUrlMode ? (
              <div className="animate-in fade-in duration-300">
                <input
                  {...register("csvUrl", { required: isUrlMode })}
                  type="url"
                  placeholder="https://example.com/data.csv"
                  className="w-full p-3 border border-slate-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 outline-none"
                />
              </div>
            ) : (
              <input
                type="file"
                {...register("csvFile", { required: !isUrlMode })}
                className="block w-full text-sm text-slate-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
              />
            )}
          </div>

          <button
            type="submit"
            className="w-full bg-slate-900 hover:bg-black text-white font-bold py-3 rounded-xl shadow-lg transition-all transform active:scale-[0.98]"
          >
            Analyze Transaction Graph
          </button>
        </form>

        <footer className="mt-8 text-center text-[10px] text-slate-400 uppercase tracking-widest">
          Engineered with Rust & Preact
        </footer>
      </div>
    </div>
  );
}