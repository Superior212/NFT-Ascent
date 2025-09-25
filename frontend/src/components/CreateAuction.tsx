import { useState, useEffect } from "react";
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
import { Badge } from "@/components/ui/badge";
import { useNFT } from "@/hooks/useNFT";
import { useMarketplace } from "@/hooks/useMarketplace";
import { useWeb3 } from "@/hooks/useWeb3";
import { CONTRACTS } from "@/config/contracts";
import { Plus, Check, X } from "lucide-react";

export const CreateAuction = () => {
  const [userNFTs, setUserNFTs] = useState<any[]>([]);
  const [selectedNFT, setSelectedNFT] = useState<string | null>(null);
  const [reservePrice, setReservePrice] = useState("");
  const [duration, setDuration] = useState("86400"); // 24 hours default
  const [approvalStatus, setApprovalStatus] = useState<Record<string, boolean>>(
    {}
  );

  const { provider, signer, account } = useWeb3();
  const {
    getOwnerNFTs,
    approveMarketplace,
    getApprovalStatus,
    loading: nftLoading,
  } = useNFT(provider, signer, account);
  const { createAuction, loading: marketplaceLoading } = useMarketplace(
    provider,
    signer
  );

  useEffect(() => {
    if (account && getOwnerNFTs) {
      console.log("Account changed, loading NFTs...");
      loadUserNFTs();
    }
  }, [account, getOwnerNFTs]);

  // Also load NFTs when component mounts if account is already available
  useEffect(() => {
    if (account && getOwnerNFTs && userNFTs.length === 0) {
      console.log("Component mounted, loading NFTs...");
      loadUserNFTs();
    }
  }, []);

  const loadUserNFTs = async () => {
    if (!account) {
      console.log("No account available");
      return;
    }

    console.log("Loading NFTs for account:", account);
    const nfts = await getOwnerNFTs(account);
    console.log("Retrieved NFTs:", nfts);
    setUserNFTs(nfts);

    // Check approval status for each NFT
    const approvals: Record<string, boolean> = {};
    for (const nft of nfts) {
      console.log(`Checking approval status for token ${nft.tokenId}`);
      approvals[nft.tokenId] = await getApprovalStatus(nft.tokenId);
    }
    console.log("Approval statuses:", approvals);
    setApprovalStatus(approvals);
  };

  const handleApprove = async (tokenId: string) => {
    const result = await approveMarketplace(tokenId);
    if (result.success) {
      setApprovalStatus((prev) => ({ ...prev, [tokenId]: true }));
      alert("NFT approved for marketplace!");
    } else {
      alert(`Approval failed: ${result.error}`);
    }
  };

  const handleCreateAuction = async () => {
    if (!selectedNFT || !reservePrice || !duration) {
      alert("Please fill in all fields");
      return;
    }

    if (!approvalStatus[selectedNFT]) {
      alert("Please approve the NFT for marketplace first");
      return;
    }

    const result = await createAuction(
      CONTRACTS.NFT_CONTRACT,
      selectedNFT,
      reservePrice,
      Number(duration)
    );

    if (result.success) {
      alert(
        `Auction created successfully! Auction ID: ${
          result.auctionId || "Unknown"
        }`
      );
      setSelectedNFT(null);
      setReservePrice("");
      setDuration("86400");
      loadUserNFTs(); // Refresh NFTs
    } else {
      alert(`Auction creation failed: ${result.error}`);
    }
  };

  if (!account) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Create Auction</CardTitle>
          <CardDescription>
            Connect your wallet to create auctions
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Plus className="h-5 w-5" />
          Create Auction
        </CardTitle>
        <CardDescription>List your NFTs for auction</CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-4">
          <div>
            <Label>Your NFTs</Label>
            <div className="grid grid-cols-1 gap-2 mt-2 max-h-40 overflow-y-auto">
              {nftLoading ? (
                <div className="flex items-center justify-center py-4">
                  <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-primary"></div>
                  <span className="ml-2 text-sm text-muted-foreground">Loading NFTs...</span>
                </div>
              ) : userNFTs.length === 0 ? (
                <p className="text-sm text-muted-foreground">No NFTs found</p>
              ) : (
                userNFTs.map((nft) => (
                  <div
                    key={nft.tokenId}
                    className={`p-3 border rounded-lg cursor-pointer transition-colors ${
                      selectedNFT === nft.tokenId
                        ? "border-primary bg-primary/5"
                        : "border-border"
                    }`}
                    onClick={() => setSelectedNFT(nft.tokenId)}>
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium">Token #{nft.tokenId}</p>
                        <p className="text-sm text-muted-foreground truncate">
                          {nft.tokenURI === "URI not available"
                            ? "Metadata not available"
                            : nft.tokenURI}
                        </p>
                      </div>
                      <div className="flex items-center gap-2">
                        {approvalStatus[nft.tokenId] ? (
                          <Badge
                            variant="default"
                            className="flex items-center gap-1">
                            <Check className="h-3 w-3" />
                            Approved
                          </Badge>
                        ) : (
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleApprove(nft.tokenId);
                            }}
                            disabled={nftLoading}
                            className="flex items-center gap-1">
                            <X className="h-3 w-3" />
                            Approve
                          </Button>
                        )}
                      </div>
                    </div>
                  </div>
                ))
              )}
            </div>
            <div className="flex gap-2 mt-2">
              <Button
                variant="outline"
                size="sm"
                onClick={loadUserNFTs}
                disabled={nftLoading}>
                {nftLoading ? "Loading..." : "Refresh NFTs"}
              </Button>
            </div>
          </div>

          {selectedNFT && (
            <>
              <div className="space-y-2">
                <Label htmlFor="reservePrice">Reserve Price (ETH)</Label>
                <Input
                  id="reservePrice"
                  type="number"
                  placeholder="0.1"
                  value={reservePrice}
                  onChange={(e) => setReservePrice(e.target.value)}
                  step="0.001"
                  min="0"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="duration">Duration (seconds)</Label>
                <Input
                  id="duration"
                  type="number"
                  placeholder="86400"
                  value={duration}
                  onChange={(e) => setDuration(e.target.value)}
                  min="3600"
                />
                <p className="text-sm text-muted-foreground">
                  Common durations: 1 hour (3600), 1 day (86400), 1 week
                  (604800)
                </p>
              </div>

              <Button
                onClick={handleCreateAuction}
                disabled={marketplaceLoading || !approvalStatus[selectedNFT]}
                className="w-full">
                {marketplaceLoading ? "Creating..." : "Create Auction"}
              </Button>
            </>
          )}
        </div>
      </CardContent>
    </Card>
  );
};
