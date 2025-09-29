import { useState, useCallback } from "react";
import { pinata } from "@/config/ipfs";

export const useIpfsUpload = () => {
  const [loadingIpfs, setLoadingIpfs] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const uploadToIpfs = useCallback(async (file: File): Promise<string | null> => {
    if (!file) {
      setError("No file provided");
      return null;
    }

    setLoadingIpfs(true);
    setError(null);

    try {
      const { cid } = await pinata.upload.public.file(file);
      if (!cid) {
        setError("Failed to upload file to IPFS");
        return null;
      }

      const ipfsUri = `ipfs://${cid.toString()}`;
      console.log("File uploaded to IPFS:", { cid: cid.toString(), uri: ipfsUri });
        return ipfsUri;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error occurred";
      setError(`IPFS upload failed: ${errorMessage}`);
      console.error("IPFS upload error:", err);
      return null;
    } finally {
      setLoadingIpfs(false);
    }
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);


  return {
    uploadToIpfs,
    loadingIpfs,
    error,
    clearError,
  };
};
