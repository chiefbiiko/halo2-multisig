import { useState, useEffect } from 'react'
import { configureStore } from '@reduxjs/toolkit'
// import {
//   Contract,
//   formatUnits,
//   parseEther,
//   keccak256,
//   formatEther
// } from 'ethers'
// import { BigNumber } from '@ethersproject/bignumber'
// import { Box, Text, Link } from 'rebass'
// import bermuda from 'bermuda-bay-sdk'
// import {
//   mpecdh,
//   calcMPECDHAddress,
//   isMPECDHDeployed,
//   isMPECDHReady
// } from 'mpecdh'
import {
//   shorten,
//   copyToClipboard,
  getSigner,
//   setIntervalX,
//   fetchGelatoRelayTx,
//   gelatoTaskUrl,
//   prettierBalance
} from './util'
// import { TxAnchor, Anchor } from './anchor'
import MetaMask from './metamask'
// import { TOKENS, FAUCET_DRIP } from './constants'
import Safe, { EthersAdapter } from "@safe-global/protocol-kit"
import SafeApiKit from "@safe-global/api-kit"
import { ethers } from "ethers"

const SIGN_MSG_LIB = "0xd53cd0aB83D845Ac265BE939c57F53AD838012c9"



const metamask = new MetaMask()
// let sdk = bermuda('sepolia')
let sdk



//✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞ TBC


// export function sign(masterSafe, oldSigner, newSigner) {
//    return async function(dispatch, getState) {
//     const msg = JSON.stringify({masterSafe, oldSigner, newSigner})

//     const signer = await getSigner()
    
//       const ethAdapter = new EthersAdapter({
//         ethers,
//         signerOrProvider: signer
//       })
    
//       const safeSigner = await Safe.create({
//         ethAdapter,
//         safeAddress: process.env.SAFE
//       })
//       const rawData = new ethers.Interface([
//         "function signMessage(bytes calldata _data)"
//       ]).encodeFunctionData("signMessage", [Buffer.from(msg, "utf8")])
//       const safeTransactionData = {
//         to: SIGN_MSG_LIB,
//         data: rawData,
//         operation: 1, // delegateCall
//         value: "0"
//       }
//       const safeTx = await safeSigner.createTransaction({
//         transactions: [safeTransactionData]
//       })
    
//       const apiKit = new SafeApiKit({
//         chainId: 100
//       })
    
//       // Deterministic hash based on transaction parameters
//       const safeTxHash = await safeSigner.getTransactionHash(safeTx)
    
//       // Sign transaction to verify that the transaction is coming from owner 1
//       const senderSignature = await safeSigner.signHash(safeTxHash)
    
// // const tx = {
// //     to: masterSafe,
// //     value: 0,
// //     data: safeTx.data
// // }

// // await signer.provider.broadcastTransaction()

//     //   await apiKit.proposeTransaction({
//     //     safeAddress: masterSafe,
//     //     safeTransactionData: safeTx.data,
//     //     safeTxHash,
//     //     senderAddress: signer.address, //await owner1Signer.getAddress(),
//     //     senderSignature: senderSignature.data
//     //   })
    
//       console.log(
//         `proposed: Safe ---delegatecall---> SignMessageLib.signMessage(${msg})`
//       )
//       console.log("safe tx hash:", safeTxHash)
//    }
// }

export async  function gen( ) {
    let masterSafe = null
    let msgHash = null
    await fetch(`http://18.198.3.160:3000/getStorageProof?masterSafeAddress=${masterSafe}&msgHash=${msgHash}`)
}

export function useMyMetaMask() {
  const [chainId, setChainId] = useState(undefined)
  const [accounts, setAccounts] = useState([])
  useEffect(() => {
    metamask.provider.on('accountsChanged', accounts => setAccounts(accounts))
    metamask.provider.on('chainChanged', chainId => setChainId(Number(chainId)))
    metamask.provider
      .request({
        method: 'wallet_getPermissions'
      })
      .then(permissions => {
        if (permissions.length) {
          metamask.getAccounts().then(accounts => setAccounts(accounts))
        }
      })
    metamask.getChainId().then(chainId => setChainId(chainId))
    return async () => {}
  }, []) // doing `}, [accounts])` was bad
  return {
    isConnected: !!accounts.length,
    chainId,
    chainName:
      chainId === 100 ? 'gnosis' : chainId === 11155111 ? 'sepolia' : undefined,
    accounts,
    account: accounts?.[0],
    provider: metamask.provider
  }
}

function onAccountsChanged(dispatch, getState, _accounts) {
  window.location.reload()
  // dispatch(
  //   dump({
  //     showConnectModal: true,
  //     showRegisterModal: false,
  //     shieldedPrivateKey: undefined,
  //     selectedMenu: null,
  //     shieldedBalance: {},
  //     standardBalance: {},
  //     shieldedAddress: null,
  //     showSecretSeedInput: false,
  //     shieldedAccountSeed: null,
  //     isRegistered: false
  //   })
  // )

  // loadBalance()(dispatch, getState)
}

// async function onChainChanged(dispatch, getState, chainId) {
//   chainId = Number(chainId)
//   const chainName =
//     chainId === 100 ? 'gnosis' : chainId === 11155111 ? 'sepolia' : undefined
//   if (chainName !== 'sepolia') {
//     dispatch(dump({ showChainModal: true, selectedChain: 'sepolia' }))
//     return
//   }

//   sdk = await bermuda(chainName, { provider: sdk.config.provider })
//   const gelatoRelayFeeEstimates = Object.fromEntries(
//     await Promise.all(
//       TOKENS.map(async token => [
//         token.symbol,
//         await sdk.utils
//           .gelatoRelayFeeEstimate(token.address[chainName], chainId, 255)
//           .then(String)
//           .catch(_ => undefined)
//       ])
//     )
//   )
//   const registrationTerm = await sdk.registry.getTerm().catch(_err => undefined)
//   dispatch(
//     dump({
//       selectedChain: chainName,
//       gelatoRelayFeeEstimates,
//       registrationTerm
//     })
//   )
//   sdk.utils.onprogress(desc => dispatch(dump({ progress: desc })))
//   sdk.registry.load()
// }

// export function estimateGelatoRelayFees() {
//   return async function (dispatch) {
//     const chainId = await metamask.getChainId()
//     const chainName = await metamask.getChainName()

//     const entries = await Promise.all(
//       TOKENS.map(async token => {
//         return [
//           token.symbol,
//           await sdk.utils
//             .gelatoRelayFeeEstimate(token.address[chainName], chainId, 255)
//             .then(String)
//             .catch(_ => undefined)
//         ]
//       })
//     )

//     const gelatoRelayFeeEstimates = Object.fromEntries(entries)

//     dispatch(dump({ gelatoRelayFeeEstimates }))
//   }
// }

// export function connect(account) {
//   return async function (dispatch, getState) {
//     metamask.provider.on(
//       'accountsChanged',
//       onAccountsChanged.bind(null, dispatch, getState)
//     )
//     metamask.provider.on(
//       'chainChanged',
//       onChainChanged.bind(null, dispatch, getState)
//     )
//     if (!account) {
//       await metamask.connect()
//     }
//     const chainId = await metamask.getChainId()
//     const chainName = await metamask.getChainName()
//     if (chainName !== 'sepolia') {
//       dispatch(
//         dump({
//           showChainModal: true,
//           selectedChain: 'sepolia',
//           showConnectModal: true,
//           showSecretSeedInput: false
//         })
//       )
//       return
//     }
//     dispatch(
//       dump({
//         showConnectModal: true,
//         showSecretSeedInput: false
//       })
//     )
//     sdk = await bermuda('sepolia')
//     const gelatoRelayFeeEstimates = Object.fromEntries(
//       await Promise.all(
//         TOKENS.map(async token =>
//           token.address[chainName]
//             ? [
//                 token.symbol,
//                 await sdk.utils
//                   .gelatoRelayFeeEstimate(
//                     token.address[chainName],
//                     chainId,
//                     255
//                   )
//                   .then(String)
//                   .catch(_ => undefined)
//               ]
//             : []
//         )
//       )
//     )

//     const registrationTerm = await sdk.registry.getTerm()
//     dispatch(dump({ gelatoRelayFeeEstimates, registrationTerm }))

//     sdk.utils.onprogress(desc => dispatch(dump({ progress: desc })))

//     await sdk.registry.load()

//     loadBalance()(dispatch, getState)
//   }
// }

export function disconnect() {
  return async function (dispatch, _getState) {
    metamask.provider.removeAllListeners('accountsChanged')
    metamask.provider.removeAllListeners('chainChanged')
    // TODO not yet killing sdk connections
    dispatch(
      dump({
        shieldedPrivateKey: undefined,
        shieldedBalance: {},
        standardBalance: {},
        shieldedAddress: null,
        showConnectModal: false,
        showSecretSeedInput: false,
        shieldedAccountSeed: null,
        selectedMenu: null,
        isRegistered: null,
        utxosh: [],
        viewingKey: undefined
      })
    )
  }
}

// export function switchNetwork() {
//   return async function (dispatch, getState) {
//     const { selectedChain, customRpcUrl } = getState()
//     // HACK try init sdk once
//     sdk = await bermuda(selectedChain)
//     await metamask.switchNetwork(selectedChain)
//     sdk = await bermuda(selectedChain, {
//       provider: customRpcUrl == undefined ? sdk.config.provider : customRpcUrl
//     })
//     const registrationTerm = await sdk.registry
//       .getTerm()
//       .catch(_err => undefined)
//     dispatch(dump({ showChainModal: false, registrationTerm }))
//     sdk.utils.onprogress(desc => dispatch(dump({ progress: desc })))
//     sdk.registry.load()
//   }
// }

// export function maxout(shielded = false) {
//   return async function (dispatch, getState) {
//     const { selectedToken, selectedTokenAddress } = getState()
//     const token = TOKENS.find(({ symbol }) => symbol === selectedToken)
//     if (!shielded) {
//       const account = await metamask.getAccounts().then(accounts => accounts[0])
//       const contract = new Contract(selectedTokenAddress, sdk.utils.ERC20_ABI, {
//         provider: sdk.config.provider
//       })
//       const balance = await contract.balanceOf(account).then(String)
//       dispatch(
//         dump({ tokenAmount: formatUnits(balance, token.decimals).slice(0, 18) })
//       )
//     } else {
//       const { shieldedBalance, selectedToken, gelatoRelayFeeEstimates } =
//         getState()
//       const balanceMinusFeeEstimate = BigNumber.from(
//         shieldedBalance[selectedToken]
//       )
//         .sub(BigNumber.from(gelatoRelayFeeEstimates[selectedToken]))
//         .toHexString()
//       dispatch(
//         dump({
//           shieldedTokenAmount: formatUnits(
//             balanceMinusFeeEstimate,
//             token.decimals
//           ).slice(0, 18)
//         })
//       )
//     }
//   }
// }

// export function register(chainId) {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true, progress: 'Registering' }))
//     let { shieldedAddress, bayNameInput, name, safeAddress, registrationFee } =
//       getState()
//     name = bayNameInput || name
//     const signer = await getSigner()
//     const _amount = parseEther(String(registrationFee || 0))
//     const registryAddress = await sdk.config.registry.getAddress()
//     const registryFeeToken = await sdk.config.registry.token()
//     const token = new Contract(registryFeeToken, sdk.utils.ERC20_ABI, {
//       provider: sdk.config.provider
//     })
//     //FIXME handle Safe allowance - will bug wen registering with name thru Safe
//     const allowance = await token.allowance(
//       await signer.getAddress(),
//       registryAddress
//     )
//     let approveReceipt
//     if (allowance < _amount) {
//       approveReceipt = await token
//         .connect(signer)
//         .approve(registryAddress, String(Number(_amount) / 1e18)) //HCK
//         .then(res => {
//           return sdk.config.provider.waitForTransaction(res.hash)
//         })
//     }
//     if (approveReceipt && approveReceipt.status === 0) {
//       dispatch(
//         dump({
//           modalTitle: '🔥 Error',
//           modalText: (
//             <Box>
//               Allowance approval of {formatEther(registrationFee)}
//               WETH for{' '}
//               <span
//                 style={{ cursor: 'grab' }}
//                 onClick={copyToClipboard.bind(null, registryAddress)}
//               >
//                 {shorten(registryAddress)}
//               </span>{' '}
//               failed <TxAnchor hash={approveReceipt?.hash} />
//             </Box>
//           ),
//           dots: false
//         })
//       )
//       sdk.utils.progress('')
//       return
//     }

//     sdk.registry
//       .register(signer, shieldedAddress, name, safeAddress)
//       .then(response => {
//         // safeTxHash
//         if (response.safeTxHash) {
//           const shortChainName =
//             chainId === 100 ? 'gno' : chainId === 11155111 ? 'sep' : ''
//           const safeWebWalletLink = `https://app.safe.global/transactions/queue?safe=${shortChainName}:${safeAddress}`

//           dispatch(
//             dump({
//               modalTitle: '💎 Success',
//               modalText: (
//                 <Box>
//                   Proposed registration of{' '}
//                   {name ? ` ${name} as alias for ` : ' '}
//                   <span
//                     style={{ cursor: 'grab' }}
//                     onClick={copyToClipboard.bind(null, shieldedAddress)}
//                   >
//                     {shorten(shieldedAddress)}
//                   </span>
//                   .{' '}
//                   {response.safeTxHash ? (
//                     <Text sx={{ marginBottom: '0.625em' }}>
//                       To complete the registration have all Safe owners confirm
//                       transaction{' '}
//                       <span
//                         style={{ cursor: 'grab' }}
//                         onClick={copyToClipboard.bind(
//                           null,
//                           response.safeTxHash
//                         )}
//                         title="Safe tx hash"
//                       >
//                         <pre style={{ display: 'inline' }}>
//                           {shorten(response.safeTxHash)}
//                         </pre>
//                       </span>
//                       , then execute it. Do this via{' '}
//                       <Link
//                         href={safeWebWalletLink}
//                         target="_blank"
//                         style={{
//                           textDecoration: 'none',
//                           cursor: 'pointer'
//                         }}
//                       >
//                         Safe's web wallet
//                       </Link>
//                       .
//                     </Text>
//                   ) : (
//                     <TxAnchor hash={response?.hash} />
//                   )}
//                 </Box>
//               ),
//               progress: '',
//               dots: false,
//               showRegisterModal: false,
//               name
//             })
//           )
//         } else {
//           dispatch(
//             dump({
//               modalTitle: '💎 Success',
//               modalText: (
//                 <Box>
//                   Registered{name ? ` ${name} as alias for ` : ' '}
//                   <span
//                     style={{ cursor: 'grab' }}
//                     onClick={copyToClipboard.bind(null, shieldedAddress)}
//                   >
//                     {shorten(shieldedAddress)}
//                   </span>{' '}
//                   <TxAnchor hash={response?.hash} />
//                 </Box>
//               ),
//               progress: '',
//               dots: false,
//               showRegisterModal: false,
//               name
//             })
//           )
//         }
//       })
//       .catch(err => {
//         console.error(err)
//         dispatch(
//           dump({
//             modalTitle: '🔥 Error',
//             modalText: (
//               <Box>
//                 Registration failed <TxAnchor hash={err?.receipt?.hash} />
//               </Box>
//             ),
//             progress: '',
//             dots: false
//           })
//         )
//       })
//   }
// }

// export function renew() {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true }))
//     let { name, bayNameInput, shieldedAddress, registrationFee } = getState()
//     name = bayNameInput || name
//     const signer = await getSigner()

//     const _amount = parseEther(registrationFee.toString())
//     const registryAddress = await sdk.config.registry.getAddress()
//     const token = new Contract(sdk.config.cbay, sdk.utils.ERC20_ABI, {
//       provider: sdk.config.provider
//     })
//     const allowance = await token.allowance(
//       await signer.getAddress(),
//       registryAddress
//     )
//     let approveReceipt
//     if (allowance < _amount) {
//       approveReceipt = await token
//         .connect(signer)
//         .approve(registryAddress, String(Number(_amount) / 1e18)) //HCK
//         .then(res => {
//           return sdk.config.provider.waitForTransaction(res.hash)
//         })
//     }
//     if (approveReceipt && approveReceipt.status === 0) {
//       dispatch(
//         dump({
//           modalTitle: '🔥 Error',
//           modalText: (
//             <Box>
//               Allowance approval of {formatEther(registrationFee)}
//               CBAY for{' '}
//               <span
//                 style={{ cursor: 'grab' }}
//                 onClick={copyToClipboard.bind(null, registryAddress)}
//               >
//                 {shorten(registryAddress)}
//               </span>{' '}
//               failed <TxAnchor hash={approveReceipt?.hash} />
//             </Box>
//           ),
//           dots: false
//         })
//       )
//       sdk.utils.progress('')
//       return
//     }

//     sdk.registry
//       .renew(signer, name)
//       .then(({ receipt }) => {
//         dispatch(
//           dump({
//             modalTitle: '💎 Success',
//             modalText: (
//               <Box>
//                 Renewed {name} as alias for{' '}
//                 <span
//                   style={{ cursor: 'grab' }}
//                   onClick={copyToClipboard.bind(null, shieldedAddress)}
//                 >
//                   {shorten(shieldedAddress)}
//                 </span>{' '}
//                 for another year
//                 <TxAnchor hash={receipt?.hash} />
//               </Box>
//             ),
//             dots: false
//           })
//         )
//       })
//       .catch(err => {
//         console.error(err)
//         dispatch(
//           dump({
//             modalTitle: '🔥 Error',
//             modalText: (
//               <Box>
//                 Renewal failed <TxAnchor hash={err?.receipt?.hash} />
//               </Box>
//             ),
//             dots: false
//           })
//         )
//       })
//   }
// }

// export function setFaucetRecipient(recipient) {
//   return async function (dispatch, getState) {
//     const { faucetRecipient, safeAddress } = getState()
//     if (faucetRecipient === undefined) {
//       const account = await metamask.getAccounts().then(accounts => accounts[0])
//       dispatch(
//         dump({ faucetRecipient: recipient || safeAddress || account || '' })
//       )
//     } else {
//       dispatch(dump({ faucetRecipient: recipient }))
//     }
//   }
// }

// export function nameChange(name) {
//   return async function (dispatch, _getState) {
//     if (name?.length >= 7 && name?.endsWith('.bay')) {
//       const registrationExpiry = await sdk.registry
//         .expiryOf(name)
//         .then(x => x.toString())
//       const registrationFee = await sdk.registry
//         .getFee(name)
//         .then(x => x.toString())
//       dispatch(dump({ registrationFee, registrationExpiry }))
//     } else if (!name) {
//       dispatch(dump({ registrationFee: 0, registrationExpiry: 0 }))
//     }
//     dispatch(dump({ bayNameInput: name }))
//   }
// }
// //TODO remove params from fund transfer andwithdraw and have them read all that from state instead
// export function fund(amount, recipient) {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true }))
//     let {
//       shieldedAddress,
//       shieldedPrivateKey,
//       fundShieldedRecipient,
//       selectedToken,
//       selectedTokenAddress,
//       transferNote
//     } = getState()
//     sdk.utils.progress('Resolving recipient')
//     const _recipient = await sdk.registry.resolveShieldedAddress(recipient)
//     fundShieldedRecipient = _recipient
//     if (!_recipient) {
//       dispatch(
//         dump({
//           modalTitle: '4🙃4',
//           modalText: (
//             <Text> Unable to resolve {recipient} to a shielded address.</Text>
//           ),
//           dots: false
//         })
//       )
//       sdk.utils.progress('')
//       return
//     }
//     recipient = _recipient
//     sdk.utils.progress('Checking allowance')
//     const _amount = parseEther(amount)
//     const keypair = new sdk.KeyPair(shieldedPrivateKey)
//     const signer = await getSigner()
//     const poolAddress = await sdk.config.pool.getAddress()
//     const token = new Contract(selectedTokenAddress, sdk.utils.ERC20_ABI, {
//       provider: sdk.config.provider
//     })
//     const allowance = await token.allowance(
//       await signer.getAddress(),
//       poolAddress
//     )
//     let approveReceipt
//     if (allowance < _amount) {
//       approveReceipt = await token
//         .connect(signer)
//         .approve(poolAddress, _amount)
//         .then(res => {
//           return sdk.config.provider.waitForTransaction(res.hash)
//         })
//     }
//     if (approveReceipt && approveReceipt.status === 0) {
//       dispatch(
//         dump({
//           modalTitle: '🔥 Error',
//           modalText: (
//             <Box>
//               Allowance approval of {amount}
//               {selectedToken} for{' '}
//               <span
//                 style={{ cursor: 'grab' }}
//                 onClick={copyToClipboard.bind(null, poolAddress)}
//               >
//                 {shorten(poolAddress)}
//               </span>{' '}
//               failed <TxAnchor hash={approveReceipt?.hash} />
//             </Box>
//           ),
//           dots: false
//         })
//       )
//       sdk.utils.progress('')
//       return
//     }
//     await sdk.omnipool
//       .fund({
//         pool: sdk.config.pool,
//         signer,
//         keypair,
//         amount,
//         recipient,
//         token: selectedTokenAddress,
//         transferNote
//       })
//       .catch(err => {
//         console.error(err)
//         dispatch(
//           dump({
//             modalTitle: '🔥 Error',
//             modalText: (
//               <Box>
//                 Funding failed <TxAnchor hash={err?.receipt?.hash} />
//               </Box>
//             ),
//             dots: false
//           })
//         )
//       })
//       .then(async ({ receipt, partialViewingKey }) => {
//         const recipientName = await sdk.registry.nameOfShieldedAddress(
//           fundShieldedRecipient || shieldedAddress
//         )
//         console.log('partialViewingKey', partialViewingKey) //TMP
//         dispatch(
//           dump({
//             modalTitle: '💎 Success',
//             modalText: (
//               <Box>
//                 <Box>
//                   Funded{' '}
//                   <span
//                     style={{ cursor: 'grab' }}
//                     onClick={copyToClipboard.bind(
//                       null,
//                       fundShieldedRecipient || shieldedAddress
//                     )}
//                   >
//                     {shorten(
//                       recipientName || fundShieldedRecipient || shieldedAddress
//                     )}
//                   </span>{' '}
//                   {prettierBalance(amount)} {selectedToken}{' '}
//                   <TxAnchor hash={receipt?.hash} />
//                 </Box>
//               </Box>
//             ),
//             dots: false
//           })
//         )
//         dispatch(deriveShieldedAccount(shieldedPrivateKey))
//       })
//   }
// }

// export function transfer(amount, recipient) {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true }))
//     const {
//       selectedToken,
//       selectedTokenAddress,
//       shieldedPrivateKey,
//       transferNote,
//       transferUseGelatoRelay,
//       safeAddress
//     } = getState()
//     sdk.utils.progress('Resolving recipient')
//     const _recipient = await sdk.registry.resolveShieldedAddress(recipient)
//     if (!_recipient) {
//       dispatch(
//         dump({
//           modalTitle: '4🙃4',
//           modalText: (
//             <Text> Unable to resolve {recipient} to a shielded address.</Text>
//           ),
//           dots: false
//         })
//       )
//       sdk.utils.progress('')
//       return
//     }
//     recipient = _recipient
//     const keypair = new sdk.KeyPair(shieldedPrivateKey)
//     const signer = await getSigner()
//     let gelatoFee
//     if (transferUseGelatoRelay) {
//       sdk.utils.progress('Fetching Gelato Relay fee')
//       gelatoFee = await sdk.utils.gelatoRelayFeeEstimate(
//         selectedTokenAddress,
//         sdk.config.chainId,
//         transferNote?.length
//       )
//     }
//     //WIP
//     if (safeAddress) {
//       //   precalc msghash  sdk.utils.safeStxHash({token, safe, value, nullifiers})
//       //TODO helper that returns minimal required number of input utxos for given target amount
//       //.... to then be used within transfer and withdraw AND here to precalc
//       //WIP const stxHash = sdk.utils.safeStxHash({token: selectedTokenAddress, safe: safeAddress, value, nullifiers})
//       //   propose safe sign msghash
//       //   link to safe web wallet
//       //   return
//     }
//     await sdk.omnipool
//       .transfer({
//         pool: sdk.config.pool,
//         signer,
//         keypair,
//         amount,
//         recipient,
//         token: selectedTokenAddress,
//         note: transferNote,
//         useGelatoRelay: transferUseGelatoRelay,
//         gelatoFee
//       })
//       .catch(err => {
//         console.error(err)
//         dispatch(
//           dump({
//             modalTitle: '🔥 Error',
//             modalText: (
//               <Box>
//                 Transfer failed <TxAnchor hash={err?.receipt?.hash} />
//               </Box>
//             ),
//             dots: false
//           })
//         )
//       })
//       .then(async ({ receipt, taskId, partialViewingKey }) => {
//         console.log('partialViewingKey', partialViewingKey) //TMP
//         const recipientName = await sdk.registry.nameOfShieldedAddress(
//           recipient
//         )
//         if (!taskId) {
//           dispatch(
//             dump({
//               modalTitle: '💎 Success',
//               modalText: (
//                 <Box>
//                   <Box>
//                     Transfered {prettierBalance(amount)} {selectedToken} to{' '}
//                     <span
//                       style={{ cursor: 'grab' }}
//                       onClick={copyToClipboard.bind(null, recipient)}
//                     >
//                       {shorten(recipientName || recipient)}
//                     </span>{' '}
//                     <TxAnchor hash={receipt?.hash} />
//                   </Box>
//                 </Box>
//               ),
//               dots: false
//             })
//           )
//           dispatch(deriveShieldedAccount(shieldedPrivateKey))
//         } else {
//           const tx = await fetchGelatoRelayTx(taskId).catch(console.error)
//           sdk.utils.progress('')
//           if (!tx) {
//             dispatch(
//               dump({
//                 modalTitle: '🔥 Error',
//                 modalText: (
//                   <Box>
//                     Transfer via Gelato Relay failed{' '}
//                     <Anchor
//                       href={gelatoTaskUrl(taskId)}
//                       text={shorten(taskId)}
//                     />
//                     . Please try again.
//                   </Box>
//                 ),
//                 dots: false
//               })
//             )
//           } else {
//             dispatch(
//               dump({
//                 modalTitle: '💎 Success',
//                 modalText: (
//                   <Box>
//                     <Box>
//                       Transfered {prettierBalance(amount)} {selectedToken} to{' '}
//                       <span
//                         style={{ cursor: 'grab' }}
//                         onClick={copyToClipboard.bind(null, recipient)}
//                       >
//                         {shorten(recipientName || recipient)}
//                       </span>{' '}
//                       via Gelato <TxAnchor hash={tx} /> 🍦
//                     </Box>
//                   </Box>
//                 ),
//                 dots: false
//               })
//             )
//             setIntervalX(
//               () => dispatch(deriveShieldedAccount(shieldedPrivateKey)),
//               5000,
//               6
//             )
//           }
//         }
//       })
//   }
// }

// export function withdraw(amount, recipient) {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true }))
//     const {
//       selectedToken,
//       selectedTokenAddress,
//       shieldedPrivateKey,
//       withdrawUseGelatoRelay,
//       withdrawUnwrap
//     } = getState()
//     sdk.utils.progress('Resolving recipient')
//     const _recipient = await sdk.registry.resolveNativeAddress(recipient)
//     if (!_recipient) {
//       dispatch(
//         dump({
//           modalTitle: '4🙃4',
//           modalText: (
//             <Text>Unable to resolve {recipient} to a native address.</Text>
//           ),
//           dots: false
//         })
//       )
//       sdk.utils.progress('')
//       return
//     }
//     recipient = _recipient
//     const keypair = new sdk.KeyPair(shieldedPrivateKey)
//     const signer = await getSigner()
//     let gelatoFee
//     if (withdrawUseGelatoRelay) {
//       sdk.utils.progress('Fetching Gelato Relay fee')
//       gelatoFee = await sdk.utils.gelatoRelayFeeEstimate(selectedTokenAddress)
//     }
//     const chainName = await metamask.getChainName()
//     await sdk.omnipool
//       .withdraw({
//         pool: sdk.config.pool,
//         signer,
//         keypair,
//         amount,
//         recipient,
//         token: selectedTokenAddress.toLowerCase(),
//         // can't use gelato on sepolia for gno
//         useGelatoRelay:
//           chainName === 'sepolia' && selectedToken === 'GNO'
//             ? undefined
//             : withdrawUseGelatoRelay,
//         gelatoFee,
//         // can only unwrap wxdai
//         unwrap: selectedToken === 'WETH' ? withdrawUnwrap : undefined
//       })
//       .catch(err => {
//         console.error(err)
//         dispatch(
//           dump({
//             modalTitle: '🔥 Error',
//             modalText: (
//               <Box>
//                 Withdrawal failed <TxAnchor hash={err?.receipt?.hash} />
//               </Box>
//             ),
//             dots: false
//           })
//         )
//       })
//       .then(async ({ receipt, taskId, partialViewingKey }) => {
//         console.log('partialViewingKey', partialViewingKey) //TMP
//         const recipientName = await sdk.registry.lookupEnsName(recipient)
//         if (!taskId) {
//           dispatch(
//             dump({
//               modalTitle: '💎 Success',
//               modalText: (
//                 <Box>
//                   <Box>
//                     Withdrew {prettierBalance(amount)} {selectedToken} to{' '}
//                     <span
//                       style={{ cursor: 'grab' }}
//                       onClick={copyToClipboard.bind(null, recipient)}
//                     >
//                       {shorten(recipientName || recipient)}
//                     </span>{' '}
//                     <TxAnchor hash={receipt?.hash} />
//                   </Box>
//                 </Box>
//               ),
//               dots: false
//             })
//           )
//           dispatch(deriveShieldedAccount(shieldedPrivateKey))
//         } else {
//           const tx = await fetchGelatoRelayTx(taskId).catch(console.error)
//           if (!tx) {
//             dispatch(
//               dump({
//                 modalTitle: '🔥 Error',
//                 modalText: (
//                   <Box>
//                     Withdrawal via Gelato Relay failed{' '}
//                     <Anchor
//                       href={gelatoTaskUrl(taskId)}
//                       text={shorten(taskId)}
//                     />
//                     . Please try again.
//                   </Box>
//                 ),
//                 dots: false
//               })
//             )
//           } else {
//             dispatch(
//               dump({
//                 modalTitle: '💎 Success',
//                 modalText: (
//                   <Box>
//                     <Box>
//                       Withdrew {prettierBalance(amount)} {selectedToken} to{' '}
//                       <span
//                         style={{ cursor: 'grab' }}
//                         onClick={copyToClipboard.bind(null, recipient)}
//                       >
//                         {shorten(recipientName || recipient)}
//                       </span>{' '}
//                       via Gelato <TxAnchor hash={tx} /> 🍦
//                     </Box>
//                   </Box>
//                 ),
//                 dots: false
//               })
//             )
//             setIntervalX(
//               () => dispatch(deriveShieldedAccount(shieldedPrivateKey)),
//               5000,
//               6
//             )
//           }
//         }
//       })
//   }
// }

// export function loadBalance() {
//   return async function (dispatch, getState) {
//     const {
//       selectedTokenAddress,
//       isLoadingUtxos,
//       selectedToken,
//       shieldedPrivateKey
//     } = getState()
//     if (!isLoadingUtxos) {
//       dispatch(dump({ isLoadingUtxos: true }))
//       const keypair = new sdk.KeyPair(shieldedPrivateKey)
//       const nonce = await sdk.config.pool.nonce(
//         keccak256(keypair.x25519.secretKey)
//       )
//       const utxos = await sdk.utils.findUtxos({
//         pool: sdk.config.pool,
//         keypair,
//         nonce,
//         tokens: [selectedTokenAddress]
//       })
//       const spend = sdk.utils.sumAmounts(
//         utxos[selectedTokenAddress.toLowerCase()]
//       )
//       dispatch(
//         dump({
//           shieldedBalance: {
//             [selectedToken]: spend.toHexString()
//           },
//           isLoadingUtxos: false
//         })
//       )
//     }

//     const token = TOKENS.find(({ symbol }) => symbol === selectedToken)
//     const balance = await new Contract(
//       token.address[await metamask.getChainName()],
//       sdk.utils.ERC20_ABI,
//       sdk.config.provider
//     )
//       .balanceOf(await getSigner().then(s => s.address))
//       .then(BigNumber.from)
//     dispatch(
//       dump({
//         standardBalance: {
//           [selectedToken]: balance.toHexString()
//         }
//       })
//     )
//   }
// }

// export function checkMPECDHStatus(safeAddress) {
//   return async function (dispatch) {
//     const signer = await getSigner()
//     const mpecdhDeployed = await isMPECDHDeployed(safeAddress, signer.provider)
//     let mpecdhReady = false
//     if (mpecdhDeployed) {
//       mpecdhReady = await isMPECDHReady(safeAddress, signer.provider)
//     }
//     let mpecdhBlocking = []
//     let mpecdhStatus = mpecdhReady === true ? 0 : 3
//     if (mpecdhDeployed && !mpecdhReady) {
//       const choreo = await mpecdh(
//         calcMPECDHAddress(safeAddress),
//         signer.provider
//       )
//       mpecdhBlocking = Array.from(await choreo.blocking())
//       mpecdhStatus = await choreo.status(signer).catch(err => {
//         // catch underflow errors wen all signers queues are empty at init
//         // console.error(err)
//         return 3
//       })
//     }
//     console.log('>>>', {
//       mpecdhDeployed,
//       mpecdhReady,
//       mpecdhBlocking,
//       mpecdhStatus
//     })
//     dispatch(
//       dump({ mpecdhDeployed, mpecdhReady, mpecdhBlocking, mpecdhStatus })
//     )
//   }
// }

// export function safeConnectFlow() {
//   return async function (dispatch, getState) {
//     const eoa = await getSigner().then(s => s.address)
//     const safes = await sdk.utils.safesOf(eoa)
//     console.log('>>> safes', safes)
//     dispatch(
//       dump({
//         showSafeSelectModal: true,
//         showSafePropDeplMPECDHModal: false,
//         safes
//       })
//     )
//   }
// }

// export function initShieldedAccount(seed) {
//   return async function (dispatch) {
//     const shieldedPrivateKey = keccak256(seed)
//     dispatch(
//       dump({
//         shieldedPrivateKey /*FIXME*/,
//         selectedMenu: 'fund',
//         modalTitle: null,
//         modalText: null,
//         showConnectModal: false,
//         dots: true,
//         progress: 'Initializing'
//       })
//     )
//     dispatch(deriveShieldedAccount(shieldedPrivateKey))
//   }
// }

// export function deriveShieldedAccount(shieldedPrivateKey) {
//   return async function (dispatch, getState) {
//     const { selectedToken, selectedTokenAddress } = getState()
//     const keypair = new sdk.KeyPair(shieldedPrivateKey)
//     const shieldedAddress = keypair.address()
//     const isRegistered = await sdk.registry.isRegistered(shieldedAddress)
//     const eoa = await getSigner().then(s => s.address)
//     let name
//     let registrationExpiry
//     let registrationFee
//     // let safes
//     if (isRegistered) {
//       const _name = await sdk.registry.nameOfShieldedAddress(shieldedAddress)
//       if (_name) {
//         registrationExpiry = await sdk.registry.expiryOf(_name)
//         registrationFee = await sdk.registry.getFee(_name)
//         name = _name
//       }
//     }
//     // else {
//     // // check if eoa has a circles safe - might be undefined
//     // safes = await circlesSafesOf(eoa)
//     // }
//     // check if eoa owns any Safes
//     // const safes = await sdk.utils.safesOf(eoa)

//     // console.log('>>> safes', safes)
//     const ensName = await sdk.registry
//       .lookupEnsName(eoa)
//       .then(n => n || undefined)

//     dispatch(
//       dump({
//         shieldedAddress,
//         isRegistered,
//         name,
//         ensName,
//         registrationExpiry,
//         registrationFee,
//         // safes,
//         dots: false,
//         progress: ''
//       })
//     )
//     const nonce = await sdk.config.pool.nonce(
//       keccak256(keypair.x25519.secretKey)
//     )
//     const utxos = await sdk.utils.findUtxos({
//       pool: sdk.config.pool,
//       keypair,
//       nonce,
//       tokens: [selectedTokenAddress]
//     })
//     const spend = sdk.utils.sumAmounts(
//       utxos[selectedTokenAddress.toLowerCase()]
//     )
//     dispatch(
//       dump({
//         shieldedBalance: {
//           [selectedToken]: spend.toHexString()
//         },
//         dots: false
//       })
//     )
//   }
// }

// export function history() {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true, progress: 'Loading transaction history' }))
//     const { shieldedPrivateKey } = getState()
//     const keypair = new sdk.KeyPair(shieldedPrivateKey)
//     await keypair.fetchNonce(sdk.config.pool)
//     const utxosh = await sdk.utils.findUtxosH({
//       pool: sdk.config.pool,
//       keypair
//     })
//     dispatch(
//       dump({
//         utxosh,
//         dots: false,
//         progress: '',
//         batchViewingKey: false,
//         viewingKey: '',
//         compoundViewingKey: ''
//       })
//     )
//   }
// }

// export function setViewingKey(viewingKey) {
//   return async function (dispatch) {
//     dispatch(
//       dump({ viewingKey, dots: true, progress: 'Loading transaction history' })
//     )
//     await sdk.registry.load()
//     const utxosh = []
//     const viewingKeys = viewingKey.replace('0x', '').match(/.{64}/g) ?? []
//     for (const k of viewingKeys) {
//       const batch = await sdk.utils.findUtxosH({
//         pool: sdk.config.pool,
//         viewingKey: k
//       })
//       Array.prototype.push.apply(utxosh, batch)
//     }
//     dispatch(dump({ utxosh, dots: false, progress: '' }))
//   }
// }

// export function genViewingKey() {
//   return async function (dispatch, getState) {
//     const { compoundViewingKey } = getState()
//     dispatch(dump({ batchViewingKey: compoundViewingKey ? false : true }))
//     if (compoundViewingKey) {
//       console.log('=== compound viewing key', compoundViewingKey)
//       dispatch(
//         dump({
//           modalTitle: '🔐 Viewing key',
//           modalText: (
//             <Box>
//               Generated viewing key{' '}
//               <span
//                 style={{ cursor: 'grab' }}
//                 onClick={copyToClipboard.bind(null, compoundViewingKey)}
//               >
//                 {shorten(compoundViewingKey)}
//               </span>
//             </Box>
//           ),
//           compoundViewingKey: ''
//         })
//       )
//       document.querySelectorAll('.key-lock').forEach(element => {
//         element.innerText = '🔐'
//         element.batched = false
//       })
//     }
//   }
// }

// export function drip() {
//   return async function (dispatch, getState) {
//     dispatch(dump({ dots: true, progress: 'Minting' }))
//     let { faucetRecipient } = getState()
//     const signer = await getSigner()
//     const amount = BigNumber.from(FAUCET_DRIP.toString())
//     sdk.config.cbay
//       .connect(signer)
//       .mint(faucetRecipient)
//       .then(receipt => {
//         dispatch(
//           dump({
//             modalTitle: '💎 Success',
//             modalText: (
//               <Box>
//                 Minted {formatEther(amount.toString())} CBAY to{' '}
//                 <span
//                   style={{ cursor: 'grab' }}
//                   onClick={copyToClipboard.bind(null, faucetRecipient)}
//                 >
//                   {shorten(faucetRecipient)}
//                 </span>{' '}
//                 <TxAnchor hash={receipt?.hash} />
//               </Box>
//             ),
//             dots: false,
//             progress: ''
//           })
//         )
//       })
//       .catch(err => {
//         console.error(err)
//         dispatch(
//           dump({
//             modalTitle: '🔥 Error',
//             modalText: (
//               <Box>
//                 Mint failed <TxAnchor hash={err?.receipt?.hash} />
//               </Box>
//             ),
//             dots: false,
//             progress: ''
//           })
//         )
//       })
//   }
// }

// export function fetchRegistry() {
//   return async function (dispatch, _getState) {
//     const registered = await sdk.registry.list()
//     const registryContacts = await Promise.all(
//       registered.map(async r => ({
//         ...r,
//         name: Buffer.from(r.name.replace('0x', ''), 'hex').toString('utf8'),
//         //  ens:await sdk.registry
//         //  .lookupEnsName(r.nativeAddress)
//         //  .then(n => n || undefined),
//         gno: await sdk.registry
//           .lookupGnoName(r.nativeAddress)
//           .then(n => n || undefined)
//       }))
//     )
//     dispatch(dump({ registryContacts }))
//   }
// }

// export function doubleDumpFrens(frens) {
//   return async function (dispatch, getState) {
//     const { shieldedAddress } = getState()
//     dispatch(dump({ frens }))
//     localStorage.setItem(
//       `bermuda_frens_${shieldedAddress}`,
//       JSON.stringify(frens)
//     )
//   }
// }

// export function resolveNames({nativeAddresses, shieldedAddresses}) {
//   return async function (dispatch, getState) {
//     let nativeNames = await Promise.all(nativeAddresses.map(a => sdk.registry.lookupEnsName(a).then(n => [a, n || a])))
//     nativeNames = Object.fromEntries(nativeNames)

//     let shieldedNames = await Promise.all(shieldedAddresses.map(a =>   sdk.registry.lookupEnsName(a).then(n => [a, n || a])   ))
//     shieldedNames = Object.fromEntries(shieldedNames)

//   }
// }

const DUMP = 'DUMP'

export function dump(props) {
  return { type: DUMP, ...props }
}

export const store = configureStore({
  reducer(
    state = {
    //   transferUseGelatoRelay: true,
    //   withdrawUseGelatoRelay: true,
    //   withdrawUnwrap: true,
    //   selectedChain: 'sepolia',
    //   selectedToken: 'WETH',
    //   selectedTokenAddress: '0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9', //WETH sepolia
    //   shieldedBalance: {},
    //   standardBalance: {},
    //   frens: []
    },
    { type, ...props }
  ) {
    // let withdrawUseGelatoRelay =
    //   props?.withdrawUseGelatoRelay ?? state.withdrawUseGelatoRelay
    // let selectedTokenAddress = state.selectedTokenAddress
    // if (props?.selectedToken) {
    //   selectedTokenAddress = TOKENS.find(
    //     ({ symbol }) => symbol === props?.selectedToken
    //   )?.address?.[props?.selectedChain || state.selectedChain]
    // }
    // let frens = props.frens || state.frens
    // if (props?.shieldedAddress) {
    //   frens =
    //     JSON.parse(
    //       localStorage.getItem(`bermuda_frens_${props.shieldedAddress}`)
    //     ) || []
    // }
    switch (type) {
      case DUMP:
        return {
          ...state,
          ...props,
        //   withdrawUseGelatoRelay,
        //   selectedTokenAddress,
        //   shieldedBalance: {
        //     ...state.shieldedBalance,
        //     ...props.shieldedBalance
        //   },
        //   standardBalance: {
        //     ...state.standardBalance,
        //     ...props.standardBalance
        //   },
        //   frens
        }
      default:
        return state
    }
  },
  middleware: getDefaultMiddleware =>
    getDefaultMiddleware({
      serializableCheck: {
        // Ignore these action types
        ignoredActions: ['DUMP'],
        // Ignore these field paths in all actions
        ignoredActionPaths: ['modalText', 'registrationFee'],
        // Ignore these paths in the state
        ignoredPaths: ['modalText', 'registrationFee']
      }
    })
})