import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useNFT } from "@/hooks/useNFT";
import { useWeb3 } from "@/hooks/useWeb3";
import { Settings } from "lucide-react";

export const InitializeNFT = () => {
  const [name, setName] = useState("Neon NFT Collection");
  const [symbol, setSymbol] = useState("NEON");
  const { provider, signer, account } = useWeb3();
  const { initializeNFT, loading, error } = useNFT(provider, signer, account);

  const handleInitialize = async () => {
    if (!name.trim() || !symbol.trim()) {
      alert("Please enter both name and symbol");
      return;
    }

    const result = await initializeNFT(name, symbol);
    if (result.success) {
      alert("NFT contract initialized successfully!");
    } else {
      alert(`Initialization failed: ${result.error}`);
    }
  };

  if (!account) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Initialize NFT Contract</CardTitle>
          <CardDescription>
            Connect your wallet to initialize the NFT contract
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Settings className="h-5 w-5" />
          Initialize NFT Contract
        </CardTitle>
        <CardDescription>
          Initialize the NFT contract with name and symbol (one-time setup)
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="name">Collection Name</Label>
          <Input
            id="name"
            placeholder="Neon NFT Collection"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="symbol">Collection Symbol</Label>
          <Input
            id="symbol"
            placeholder="NEON"
            value={symbol}
            onChange={(e) => setSymbol(e.target.value)}
          />
        </div>

        <Button
          onClick={handleInitialize}
          disabled={loading || !name.trim() || !symbol.trim()}
          className="w-full">
          {loading ? "Initializing..." : "Initialize Contract"}
        </Button>

        {error && <p className="text-sm text-red-500">{error}</p>}
      </CardContent>
    </Card>
  );
};
