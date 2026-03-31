import { useState } from 'preact/hooks';
import { Copy, Check } from 'lucide-react';

export const ExampleData = () => {
  const [activeCopy, setActiveCopy] = useState(null);

  const exampleData = {
    initial_wallet: "TL0_Node7_x92Jk",
    final_wallet: "TL4_Node5_wQ11",
    csv_url: "https://raw.githubusercontent.com/LuisErnestoZamb/ladroncito/refs/heads/main/data/transacciones_complejas.csv"
  };

  const copyToClipboard = async (text, key) => {
    try {
      await navigator.clipboard.writeText(text);
      setActiveCopy(key);
      setTimeout(() => setActiveCopy(null), 1500);
    } catch (err) {
      console.error('Failed to copy: ', err);
    }
  };

  return (
    <div className="mt-10 bg-slate-800 border border-slate-700 rounded-lg overflow-hidden">
      <div className="bg-slate-800 px-4 py-2 border-b border-slate-700">
        <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">
          Test Dataset
        </h3>
      </div>

      <div className="p-2 space-y-1">
        {Object.entries(exampleData).map(([key, value]) => (
          <div key={key} className="flex items-center justify-between gap-4 p-2 hover:bg-slate-800/50 rounded group transition-colors">
            <div className="flex-1 min-w-0">
              <p className="text-[10px] text-slate-300 font-mono uppercase">{key.replace('_', ' ')}</p>
              <p className="text-sm font-mono text-blue-300 truncate">{value}</p>
            </div>

            <button
              onClick={() => copyToClipboard(value, key)}
              className="flex items-center gap-2 px-3 py-1.5 rounded bg-slate-800 border border-slate-600 text-slate-300 hover:text-white hover:border-slate-400 transition-all text-xs"
            >
              {activeCopy === key ? (
                <Check size={14} className="text-green-400" />
              ) : (
                <Copy size={14} className="opacity-60 group-hover:opacity-100" />
              )}
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};