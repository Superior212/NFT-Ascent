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
import { ImageIcon } from "lucide-react";

export const MintNFT = () => {
  const [tokenURI, setTokenURI] = useState("");
  const { provider, signer, account } = useWeb3();
  const { mintNFT, loading, error } = useNFT(provider, signer, account);

  const handleMint = async () => {
    if (!tokenURI.trim()) {
      alert("Please enter a token URI");
      return;
    }

    const result = await mintNFT(tokenURI);
    if (result.success) {
      alert(
        `NFT minted successfully! Token ID: ${result.tokenId || "Unknown"}`
      );
      setTokenURI("");
    } else {
      alert(`Minting failed: ${result.error}`);
    }
  };

  if (!account) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Mint NFT</CardTitle>
          <CardDescription>Connect your wallet to mint NFTs</CardDescription>
        </CardHeader>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <ImageIcon className="h-5 w-5" />
          Mint NFT
        </CardTitle>
        <CardDescription>
          Create a new NFT with IPFS metadata URI
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="tokenURI">Token URI (IPFS)</Label>
          <Input
            id="tokenURI"
            placeholder="ipfs://QmYourMetadataHash"
            value={tokenURI}
            onChange={(e) => setTokenURI(e.target.value)}
          />
          <p className="text-sm text-muted-foreground">
            Enter an IPFS URI pointing to your NFT metadata JSON
          </p>
        </div>

        <Button
          onClick={handleMint}
          disabled={loading || !tokenURI.trim()}
          className="w-full">
          {loading ? "Minting..." : "Mint NFT"}
        </Button>

        {error && <p className="text-sm text-red-500">{error}</p>}
      </CardContent>
    </Card>
  );
};
