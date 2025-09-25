import { useState, useEffect } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useMarketplace } from "@/hooks/useMarketplace";
import { useWeb3 } from "@/hooks/useWeb3";
import { AuctionCard } from "./AuctionCard";
import { MintNFT } from "./MintNFT";
import { CreateAuction } from "./CreateAuction";
import { RefreshCw, Store } from "lucide-react";

export const Marketplace = () => {
  const [auctions, setAuctions] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  const { provider, signer, account } = useWeb3();
  const { getActiveAuctions } = useMarketplace(provider, signer);

  const loadAuctions = async () => {
    setLoading(true);
    try {
      const activeAuctions = await getActiveAuctions();
      setAuctions(activeAuctions);
    } catch (error) {
      console.error("Failed to load auctions:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadAuctions();
  }, [account]);

  return (
    <div className="container mx-auto p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            <Store className="h-8 w-8" />
            Neon NFT Marketplace
          </h1>
          <p className="text-muted-foreground">
            Trade NFTs with English auctions on Arbitrum Stylus
          </p>
        </div>
        <Button
          variant="outline"
          onClick={loadAuctions}
          disabled={loading}
          className="flex items-center gap-2">
          <RefreshCw className={`h-4 w-4 ${loading ? "animate-spin" : ""}`} />
          Refresh
        </Button>
      </div>

      <Tabs defaultValue="marketplace" className="w-full">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="marketplace">Marketplace</TabsTrigger>
          <TabsTrigger value="mint">Mint NFT</TabsTrigger>
          <TabsTrigger value="create">Create Auction</TabsTrigger>
        </TabsList>

        <TabsContent value="marketplace" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Active Auctions</CardTitle>
              <CardDescription>
                {auctions.length} auction{auctions.length !== 1 ? "s" : ""}{" "}
                available
              </CardDescription>
            </CardHeader>
            <CardContent>
              {loading ? (
                <div className="text-center py-8">
                  <RefreshCw className="h-8 w-8 animate-spin mx-auto mb-2" />
                  <p>Loading auctions...</p>
                </div>
              ) : auctions.length === 0 ? (
                <div className="text-center py-8">
                  <Store className="h-12 w-12 mx-auto mb-4 text-muted-foreground" />
                  <p className="text-muted-foreground">
                    No active auctions found
                  </p>
                  <p className="text-sm text-muted-foreground">
                    Create an auction to get started
                  </p>
                </div>
              ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {auctions.map((auction) => (
                    <AuctionCard
                      key={auction.auctionId}
                      auction={auction}
                      onUpdate={loadAuctions}
                    />
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="mint">
          <MintNFT />
        </TabsContent>

        <TabsContent value="create">
          <CreateAuction />
        </TabsContent>
      </Tabs>
    </div>
  );
};
