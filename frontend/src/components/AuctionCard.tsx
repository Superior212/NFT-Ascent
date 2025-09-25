import { useState, useEffect } from "react";
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
import { useNFT } from "@/hooks/useNFT";
import { formatAddress, formatTime, isAuctionEnded } from "@/lib/utils";
import { Clock, Gavel, Image as ImageIcon } from "lucide-react";

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
  const [nftMetadata, setNftMetadata] = useState<{
    name?: string;
    image?: string;
    description?: string;
  } | null>(null);
  const [loadingMetadata, setLoadingMetadata] = useState(false);

  const { provider, signer, account } = useWeb3();
  const { placeBid, settleAuction, loading } = useMarketplace(provider, signer);
  const { getTokenURI } = useNFT(provider, signer, account);

  // Array of placeholder images
  const placeholderImages = [
    "https://picsum.photos/300/300?random=1",
    "https://picsum.photos/300/300?random=2",
    "https://picsum.photos/300/300?random=3",
    "https://picsum.photos/300/300?random=4",
    "https://picsum.photos/300/300?random=5",
  ];

  // Get a consistent random image based on token ID
  const getPlaceholderImage = (tokenId: string) => {
    const index = parseInt(tokenId) % placeholderImages.length;
    return placeholderImages[index];
  };

  const isEnded = isAuctionEnded(auction.endTime);
  const isOwner = account?.toLowerCase() === auction.seller.toLowerCase();
  const isHighestBidder =
    account?.toLowerCase() === auction.currentBidder.toLowerCase();
  const minBid =
    Number(auction.currentBid) > 0
      ? (Number(auction.currentBid) * 1.05).toFixed(4)
      : auction.reservePrice;

  // Fetch NFT metadata
  useEffect(() => {
    const fetchMetadata = async () => {
      if (!getTokenURI) {
        console.log(
          `No getTokenURI function available for token ${auction.tokenId}`
        );
        return;
      }

      setLoadingMetadata(true);
      try {
        console.log(`Starting metadata fetch for token ${auction.tokenId}...`);
        const tokenURI = await getTokenURI(auction.tokenId);
        console.log(`Token ${auction.tokenId} - Raw URI:`, tokenURI);

        if (!tokenURI) {
          console.log(`Token ${auction.tokenId} - No URI returned`);
          setNftMetadata(null);
          return;
        }

        if (tokenURI === "URI not available") {
          console.log(`Token ${auction.tokenId} - URI not available`);
          setNftMetadata(null);
          return;
        }

        // Handle IPFS URLs
        const metadataUrl = tokenURI.startsWith("ipfs://")
          ? `https://ipfs.io/ipfs/${tokenURI.slice(7)}`
          : tokenURI;

        console.log(
          `Token ${auction.tokenId} - Fetching from URL:`,
          metadataUrl
        );

        const response = await fetch(metadataUrl);
        console.log(
          `Token ${auction.tokenId} - Response status:`,
          response.status
        );

        if (response.ok) {
          const metadata = await response.json();
          console.log(`Token ${auction.tokenId} - Metadata:`, metadata);
          console.log(`Token ${auction.tokenId} - Image URL:`, metadata.image);
          setNftMetadata(metadata);
        } else {
          console.log(
            `Token ${auction.tokenId} - Failed to fetch metadata. Status: ${response.status}`
          );
          setNftMetadata(null);
        }
      } catch (error) {
        console.error(
          `Token ${auction.tokenId} - Error fetching metadata:`,
          error
        );
        setNftMetadata(null);
      } finally {
        setLoadingMetadata(false);
      }
    };

    fetchMetadata();
  }, [auction.tokenId, getTokenURI]);

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
          <CardTitle className="text-lg">
            {nftMetadata?.name || `Neon NFT #${auction.tokenId}`}
          </CardTitle>
          <Badge variant={isEnded ? "destructive" : "default"}>
            {isEnded ? "Ended" : "Active"}
          </Badge>
        </div>
        <CardDescription>
          Seller: {formatAddress(auction.seller)}
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* NFT Image */}
        <div className="w-full h-48 bg-muted rounded-lg flex items-center justify-center overflow-hidden relative">
          {loadingMetadata ? (
            <div className="flex flex-col items-center gap-2">
              <ImageIcon className="h-8 w-8 animate-pulse" />
              <p className="text-sm text-muted-foreground">Loading image...</p>
            </div>
          ) : nftMetadata?.image ? (
            <img
              src={
                nftMetadata.image.startsWith("ipfs://")
                  ? `https://ipfs.io/ipfs/${nftMetadata.image.slice(7)}`
                  : nftMetadata.image
              }
              alt={nftMetadata.name || `Token #${auction.tokenId}`}
              className="w-full h-full object-cover"
              onLoad={() => {
                console.log(
                  `Token ${auction.tokenId} - Image loaded successfully`
                );
              }}
              onError={(e) => {
                console.log(
                  `Token ${auction.tokenId} - Image failed to load:`,
                  nftMetadata.image
                );
                console.log(
                  `Token ${auction.tokenId} - Final image URL:`,
                  e.currentTarget.src
                );
                e.currentTarget.style.display = "none";
              }}
            />
          ) : (
            <>
              {/* Placeholder Image */}
              <img
                src={getPlaceholderImage(auction.tokenId)}
                alt={`Neon NFT #${auction.tokenId}`}
                className="w-full h-full object-cover"
                onError={(e) => {
                  console.log(
                    `Placeholder image failed to load for token ${auction.tokenId}`
                  );
                  e.currentTarget.style.display = "none";
                }}
              />
              {/* Overlay with token info */}
              <div className="absolute inset-0 bg-black/20 flex items-end">
                <div className="p-3 text-white">
                  <div className="flex items-center gap-2">
                    <div className="w-8 h-8 bg-white/20 rounded-full flex items-center justify-center">
                      <span className="text-sm font-bold">
                        #{auction.tokenId}
                      </span>
                    </div>
                    <div>
                      <p className="text-sm font-medium">Neon NFT</p>
                    </div>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>

        {/* NFT Description */}
        {nftMetadata?.description && (
          <p className="text-sm text-muted-foreground line-clamp-2">
            {nftMetadata.description}
          </p>
        )}
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
