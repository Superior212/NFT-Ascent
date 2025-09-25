import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { useMarketplace } from "@/hooks/useMarketplace";
import { useWeb3 } from "@/hooks/useWeb3";
import { formatAddress, formatTime, isAuctionEnded } from "@/lib/utils";
import { Clock, Gavel } from "lucide-react";

interface AuctionCardProps {
  auction: {
    auctionId: string;
    nftContract: string;
    tokenId: string;
    seller: string;
    reservePrice: string;
    currentBid: string;
    currentBidder: string;
    endTime: string;
    settled: boolean;
  };
  onUpdate: () => void;
}

export const AuctionCard = ({ auction, onUpdate }: AuctionCardProps) => {
  const [bidAmount, setBidAmount] = useState("");
  const { provider, signer, account } = useWeb3();
  const { placeBid, settleAuction, loading } = useMarketplace(provider, signer);

  const isEnded = isAuctionEnded(auction.endTime);
  const isOwner = account?.toLowerCase() === auction.seller.toLowerCase();
  const isHighestBidder =
    account?.toLowerCase() === auction.currentBidder.toLowerCase();
  const minBid =
    Number(auction.currentBid) > 0
      ? (Number(auction.currentBid) * 1.05).toFixed(4)
      : auction.reservePrice;

  const handleBid = async () => {
    if (!bidAmount || Number(bidAmount) <= Number(auction.currentBid)) {
      alert("Bid must be higher than current bid");
      return;
    }

    const result = await placeBid(auction.auctionId, bidAmount);
    if (result.success) {
      alert("Bid placed successfully!");
      setBidAmount("");
      onUpdate();
    } else {
      alert(`Bid failed: ${result.error}`);
    }
  };

  const handleSettle = async () => {
    const result = await settleAuction(auction.auctionId);
    if (result.success) {
      alert("Auction settled successfully!");
      onUpdate();
    } else {
      alert(`Settlement failed: ${result.error}`);
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="text-lg">Token #{auction.tokenId}</CardTitle>
          <Badge variant={isEnded ? "destructive" : "default"}>
            {isEnded ? "Ended" : "Active"}
          </Badge>
        </div>
        <CardDescription>
          Seller: {formatAddress(auction.seller)}
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <p className="text-muted-foreground">Reserve Price</p>
            <p className="font-semibold">{auction.reservePrice} ETH</p>
          </div>
          <div>
            <p className="text-muted-foreground">Current Bid</p>
            <p className="font-semibold">
              {auction.currentBid} ETH
              {auction.currentBidder !==
                "0x0000000000000000000000000000000000000000" && (
                <span className="text-xs text-muted-foreground ml-1">
                  by {formatAddress(auction.currentBidder)}
                </span>
              )}
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Clock className="h-4 w-4" />
          Ends: {formatTime(auction.endTime)}
        </div>

        {!isEnded && !isOwner && (
          <div className="space-y-2">
            <div className="flex gap-2">
              <Input
                type="number"
                placeholder={`Min: ${minBid} ETH`}
                value={bidAmount}
                onChange={(e) => setBidAmount(e.target.value)}
                step="0.001"
                min={minBid}
              />
              <Button
                onClick={handleBid}
                disabled={loading || !bidAmount}
                className="flex items-center gap-1">
                <Gavel className="h-4 w-4" />
                Bid
              </Button>
            </div>
            <p className="text-xs text-muted-foreground">
              Minimum bid: {minBid} ETH
            </p>
          </div>
        )}

        {isEnded && (isOwner || isHighestBidder) && (
          <Button onClick={handleSettle} disabled={loading} className="w-full">
            {loading ? "Settling..." : "Settle Auction"}
          </Button>
        )}

        {isOwner && !isEnded && (
          <p className="text-sm text-muted-foreground text-center">
            You are the seller of this auction
          </p>
        )}
      </CardContent>
    </Card>
  );
};
