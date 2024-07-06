import _MetaMask from '@metamask/sdk'

export default class MetaMask {
  constructor() {
    this.provider = new _MetaMask({}).getProvider() // same as window.ethereum
  }

  get isConnected() {
    return this.provider.isConnected()
  }

  async getChainId() {
    return this.provider.request({ method: 'eth_chainId' }).then(Number)
  }

  async getChainName() {
    const chainId = await this.getChainId()
    return chainId === 100
      ? 'gnosis'
      : chainId === 11155111
      ? 'sepolia'
      : undefined
  }

  async getAccounts() {
    return this.provider.request({ method: 'eth_requestAccounts' })
  }

  async connect() {
    return this.provider
      .request({ method: 'eth_requestAccounts' })
      .catch(error => {
        if (error.code === 4001) {
          // EIP-1193 userRejectedRequest error
          console.log('Please connect to MetaMask.')
        } else {
          console.error(error)
        }
      })
  }

  async switchNetwork(chain = 'sepolia') {
    const chainId =
      typeof chain === 'number' ? chain : chain === 'gnosis' ? 100 : 11155111
    try {
      await this.provider.request({
        method: 'wallet_switchEthereumChain',
        params: [{ chainId: `0x${chainId.toString(16)}` }]
      })
    } catch (switchError) {
      // 4902 indicates that the chain has not been added to MetaMask
      if (switchError.code === 4902) {
        try {
          await this.provider.request({
            method: 'wallet_addEthereumChain',
            params: [
              {
                chainId: `0x${chainId.toString(16)}`,
                chainName: chainId === 100 ? 'Gnosis' : 'Sepolia',
                rpcUrls: [
                  chainId === 100
                    ? 'https://rpc.eu-central-2.gateway.fm/v4/gnosis/archival/mainnet?apiKey=RBQ1ygy4IhH0K00AEViZOYtQIzEKAHPN.wyPL3JGGn5GJGbnv'
                    : 'https://rpc.sepolia.dev'
                ]
              }
            ]
          })
        } catch (addError) {
          console.error(addError)
        }
      }
      console.error(switchError)
    }
  }
}