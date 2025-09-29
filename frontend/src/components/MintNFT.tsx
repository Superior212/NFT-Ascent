import {useEffect, useRef, useState} from "react";
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
import { useIpfsUpload } from "@/hooks/useIpfsUpload";
import {ImageIcon, XCircle} from "lucide-react";
import {pinata} from "@/config/ipfs.ts";

export const MintNFT = () => {
  const [tokenURI, setTokenURI] = useState("");
  const [tokenImg, setTokenImg] = useState<File | null>(null);
  const [imgPreviewUrl, setImgPreviewUrl] = useState("");
  const { provider, signer, account } = useWeb3();
  const { mintNFT, loading, error } = useNFT(provider, signer, account);
  const { uploadToIpfs, loadingIpfs, error: ipfsError, clearError } = useIpfsUpload();

  const fileInputRef = useRef<HTMLInputElement>(null);

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
      setTokenImg(null);
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    } else {
      alert(`Minting failed: ${result.error}`);
    }
  };

  const handleIPFSUpload = async () => {
    if (!tokenImg) return;

    const ipfsUri = await uploadToIpfs(tokenImg);
    if (ipfsUri) {
      setTokenURI(ipfsUri);
        if (fileInputRef.current) {
            fileInputRef.current.value = '';
        }
    }
  };

  const handleRemoveFile = () => {
    setTokenImg(null);
    setTokenURI("");
    clearError();
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

    useEffect(() => {
        (async function () {
            if (!tokenURI.trim()) return "";
            const cid = tokenURI.replace("ipfs://", "");
            const url = await pinata.gateways.public.convert(cid);
            return String(url);
        })().then(setImgPreviewUrl).catch(() => setImgPreviewUrl(""));
    }, [tokenURI]);

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
              <Label htmlFor="tokenImg">Upload Image</Label>
              <div className="flex items-center gap-4">
                  <Input
                      id="tokenImg"
                      type="file"
                      placeholder="ipfs://QmYourMetadataHash"
                      accept="image/*"
                      className="w-max cursor-pointer"
                      onChange={(e) => setTokenImg(e.target.files?.[0] ?? null)}
                      ref={fileInputRef}
                  />
                  {tokenImg && <>
                      <Button
                          onClick={handleIPFSUpload}
                          disabled={loadingIpfs || !fileInputRef?.current?.value}
                          className="cursor-pointer flex items-center gap-1 bg-green-500 hover:bg-green-400 hover:text-black">
                          {loadingIpfs ? "Uploading..." : "Upload"}
                      </Button>
                      <XCircle
                          className="hover:text-red-400 cursor-pointer hover:scale-110"
                          onClick={handleRemoveFile}
                      />
                  </>}
              </div>

              {ipfsError && (
                <p className="text-sm text-red-500">{ipfsError}</p>
              )}

              <p className="text-sm text-muted-foreground">
                  Upload an Image
              </p>
          </div>
          <div className="space-y-8 text-lg font-bold text-center">OR</div>
        <div className="space-y-2">
          <Label htmlFor="tokenURI">Token URI (IPFS)</Label>
          <Input
            id="tokenURI"
            placeholder="ipfs://QmYourMetadataHash"
            value={tokenURI}
            onChange={(e) => setTokenURI(e.target.value)}
            disabled={!!tokenImg}
          />
          <p className="text-sm text-muted-foreground">
            Enter an IPFS URI pointing to your NFT metadata JSON
          </p>
        </div>
          <div className="space-y-4">
              <p>Image Preview</p>
              {imgPreviewUrl && <img src={imgPreviewUrl} alt={"From IPFS"} className=" w-full max-w-[500px]"/>}
          </div>

        <Button
          onClick={handleMint}
          disabled={loading || loadingIpfs || !tokenURI.trim()}
          className="w-full">
          {loading ? "Minting..." : "Mint NFT"}
        </Button>

        {error && <p className="text-sm text-red-500">{error}</p>}
      </CardContent>
    </Card>
  );
};
