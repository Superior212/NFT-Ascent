import { Marketplace } from "@/components/Marketplace";
import { WalletConnect } from "@/components/WalletConnect";

function App() {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <h1 className="text-xl font-bold">Neon NFT Marketplace</h1>
            </div>
            <WalletConnect />
          </div>
        </div>
      </header>
      <main>
        <Marketplace />
      </main>
    </div>
  );
}

export default App;
