# Neon NFT Marketplace Frontend

A modern, responsive frontend for the Neon NFT Marketplace built with Next.js, TypeScript, and shadcn/ui components.

## 🚀 Features

- **Modern UI**: Built with shadcn/ui components and Tailwind CSS
- **Web3 Integration**: Full integration with MetaMask and Web3 wallets
- **Real-time Updates**: Live auction data and bidding
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Type Safety**: Full TypeScript support
- **Optimized Performance**: Next.js App Router and React Query

## 🛠️ Tech Stack

- **Framework**: Next.js 14 with App Router
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **UI Components**: shadcn/ui
- **Web3**: ethers.js
- **State Management**: React hooks
- **Icons**: Lucide React

## 📦 Installation

1. **Clone and navigate to the frontend directory:**

   ```bash
   cd neon-marketplace-frontend
   ```

2. **Install dependencies:**

   ```bash
   npm install
   ```

3. **Configure contract addresses:**
   Update `src/config/contracts.ts` with your deployed contract addresses:

   ```typescript
   export const CONTRACTS = {
     NFT_CONTRACT: "0xYourDeployedNFTAddress",
     MARKETPLACE_CONTRACT: "0xYourDeployedMarketplaceAddress",
     // ... rest of config
   };
   ```

4. **Start the development server:**

   ```bash
   npm run dev
   ```

5. **Open your browser:**
   Navigate to [http://localhost:3000](http://localhost:3000)

## 🔧 Configuration

### Contract Addresses

Before using the frontend, you need to deploy your contracts and update the configuration:

1. Deploy the NFT contract:

   ```bash
   cd ../neon-nft
   cargo stylus deploy --endpoint https://stylus-testnet.arbitrum.io/rpc --private-key YOUR_PRIVATE_KEY
   ```

2. Deploy the marketplace contract:

   ```bash
   cd ../neon-marketplace
   cargo stylus deploy --endpoint https://stylus-testnet.arbitrum.io/rpc --private-key YOUR_PRIVATE_KEY
   ```

3. Initialize both contracts after deployment

4. Update `src/config/contracts.ts` with the deployed addresses

### Network Configuration

The frontend is configured for Arbitrum Sepolia testnet by default. To change networks, update the `NETWORK` configuration in `src/config/contracts.ts`.

## 🎯 Usage

### Connecting Wallet

1. Click "Connect Wallet" in the top right
2. Select MetaMask or your preferred wallet
3. Approve the connection

### Minting NFTs

1. Go to the "Mint NFT" tab
2. Enter an IPFS metadata URI (e.g., `ipfs://QmYourMetadataHash`)
3. Click "Mint NFT"
4. Confirm the transaction in your wallet

### Creating Auctions

1. Go to the "Create" tab
2. Click "Load NFTs" to see your owned NFTs
3. Select an NFT to auction
4. Approve the NFT for marketplace (one-time per NFT)
5. Set reserve price and duration
6. Click "Create Auction"

### Bidding on Auctions

1. Browse active auctions in the "Marketplace" tab
2. Click on an auction card
3. Enter your bid amount (must be higher than current bid)
4. Click the bid button
5. Confirm the transaction

### Settling Auctions

- After an auction ends, the seller or highest bidder can settle it
- Click "Settle Auction" to finalize the sale
- The NFT will be transferred to the winner
- The seller receives the bid amount minus platform fees

## 🏗️ Project Structure

```
src/
├── app/                    # Next.js App Router
│   ├── globals.css        # Global styles
│   ├── layout.tsx         # Root layout
│   └── page.tsx           # Home page
├── components/            # React components
│   ├── ui/               # shadcn/ui components
│   ├── AuctionCard.tsx   # Auction display component
│   ├── CreateAuction.tsx # Auction creation form
│   ├── MintNFT.tsx       # NFT minting form
│   └── WalletConnect.tsx # Wallet connection
├── config/               # Configuration files
│   └── contracts.ts      # Contract addresses and ABIs
├── hooks/                # Custom React hooks
│   ├── useMarketplace.ts # Marketplace operations
│   ├── useNFT.ts         # NFT operations
│   └── useWeb3.ts        # Web3 connection
└── lib/                  # Utility functions
    ├── utils.ts          # General utilities
    └── web3.ts           # Web3 utilities
```

## 🎨 UI Components

The frontend uses shadcn/ui components for a consistent, modern design:

- **Cards**: Display auction information
- **Buttons**: Interactive elements
- **Inputs**: Form controls
- **Badges**: Status indicators
- **Tabs**: Navigation between sections
- **Dialogs**: Modal interactions

## 🔒 Security Features

- **Input Validation**: All user inputs are validated
- **Error Handling**: Comprehensive error handling and user feedback
- **Transaction Safety**: Clear transaction confirmations
- **Wallet Integration**: Secure wallet connection and management

## 📱 Responsive Design

The frontend is fully responsive and works on:

- Desktop computers
- Tablets
- Mobile phones

## 🚀 Deployment

### Vercel (Recommended)

1. Push your code to GitHub
2. Connect your repository to Vercel
3. Deploy with default settings

### Other Platforms

The app can be deployed to any platform that supports Next.js:

- Netlify
- AWS Amplify
- Railway
- DigitalOcean App Platform

## 🔧 Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run lint` - Run ESLint

### Adding New Components

1. Use shadcn/ui CLI to add components:

   ```bash
   npx shadcn@latest add [component-name]
   ```

2. Create custom components in `src/components/`

### Styling

- Use Tailwind CSS classes for styling
- Follow the existing design patterns
- Use shadcn/ui components as base components

## 🐛 Troubleshooting

### Common Issues

1. **Wallet not connecting**: Ensure MetaMask is installed and unlocked
2. **Transaction failing**: Check you have enough ETH for gas fees
3. **Contract not found**: Verify contract addresses in config
4. **Network issues**: Ensure you're on the correct network (Arbitrum Sepolia)

### Getting Help

- Check the browser console for error messages
- Verify contract deployment and initialization
- Ensure you have the latest version of MetaMask

## 📄 License

MIT License - see LICENSE file for details

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

---

**Built with ❤️ using Next.js, TypeScript, and shadcn/ui**
