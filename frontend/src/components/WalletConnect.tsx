import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { useWeb3 } from "@/hooks/useWeb3";
import { formatAddress } from "@/lib/utils";
import { Wallet, LogOut } from "lucide-react";

export const WalletConnect = () => {
  const {
    isConnected,
    account,
    chainId,
    isLoading,
    error,
    connectWallet,
    disconnectWallet,
    switchNetwork,
  } = useWeb3();

  // Show loading state while checking connection
  if (isLoading && !isConnected) {
    return (
      <Button disabled className="flex items-center gap-2">
        <Wallet className="h-4 w-4" />
        Connecting...
      </Button>
    );
  }

  const handleConnect = async () => {
    await connectWallet();
    if (chainId !== 421614) {
      await switchNetwork(421614);
    }
  };

  if (isConnected && account) {
    return (
      <div className="flex items-center gap-2">
        <Badge variant="secondary" className="flex items-center gap-1">
          <Wallet className="h-3 w-3" />
          {formatAddress(account)}
        </Badge>
        <Button
          variant="outline"
          size="sm"
          onClick={disconnectWallet}
          className="flex items-center gap-1">
          <LogOut className="h-3 w-3" />
          Disconnect
        </Button>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2">
      <Button
        onClick={handleConnect}
        disabled={isLoading}
        className="flex items-center gap-2">
        <Wallet className="h-4 w-4" />
        {isLoading ? "Connecting..." : "Connect Wallet"}
      </Button>
      {error && <p className="text-sm text-red-500">{error}</p>}
    </div>
  );
};
