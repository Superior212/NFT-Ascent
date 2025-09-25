import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatEther(value: string | bigint): string {
  return (Number(value) / 1e18).toFixed(4);
}

export function parseEther(value: string): bigint {
  return BigInt(Math.floor(Number(value) * 1e18));
}

export function formatAddress(address: string): string {
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

export function formatTime(timestamp: string | number): string {
  const date = new Date(Number(timestamp) * 1000);
  return date.toLocaleString();
}

export function isAuctionEnded(endTime: string | number): boolean {
  return Date.now() / 1000 > Number(endTime);
}
