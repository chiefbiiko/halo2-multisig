import React from 'react'
import { Flex, Box, Image } from 'rebass'
import { useSelector, useDispatch } from 'react-redux'
import { formatEther } from 'ethers'
import ConnectButton from './connect-button'
// import RefreshBalanceButton from './refresh-balance-button'
import { dump, useMyMetaMask } from './redux'
import { shorten, copyToClipboard, titleCase, prettierBalance } from './util'

export default function Header() {
  const { account, chainName } = useMyMetaMask()
  let {
    shieldedAddress,
    shieldedBalance,
    showChainModal,
    selectedToken,
    name,
    ensName,
    safeAddress
  } = useSelector(state => state)
  const dispatch = useDispatch()
  shieldedBalance = shieldedBalance || { [selectedToken]: '0x00' }
  const shieldedInfo = shieldedAddress ? (
    <span>
      {' '}
      |{' '}
      <span
        style={{ cursor: 'grab' }}
        onClick={copyToClipboard.bind(null, shieldedAddress)}
        title="Shielded address"
      >
        {shorten(name || shieldedAddress)}{' '}
      </span>
      |
      <span
        style={{ cursor: 'pointer' }}
        onClick={() => dispatch(dump({ showWalletModal: true }))}
        title="Shielded balance"
      >
        {' '}
        {prettierBalance(
          formatEther(String(shieldedBalance[selectedToken] || 0))
        )}{' '}
        {selectedToken}
      </span>{' '}
      {/* <RefreshBalanceButton></RefreshBalanceButton> */}
    </span>
  ) : (
    ''
  )
  const accountInfo = (
    <span>
      <span
        style={{ cursor: 'pointer' }}
        onClick={() => dispatch(dump({ showChainModal: !showChainModal }))}
      >
        {titleCase(chainName)} |{' '}
      </span>
      <span
        style={{ cursor: 'grab' }}
        onClick={copyToClipboard.bind(null, account)}
        title="Connected account"
      >
        {shorten(safeAddress || ensName || account)}
      </span>
      {shieldedInfo}
    </span>
  )
  return (
    <Box
      style={{
        margin: '0.625em 0',
        padding: '0 0.625em'
      }}
    >
      <Flex>
        {/* <Image
          src={'/img/bermudalogobw.svg'}
          sx={{
            margin: ['0.3125em 0.625em 0 0'],
            minWidth: '11.6875em',
            width: ['15em', '16.875em', '18.75em']
          }}
        ></Image> */}
        <b style={{fontSize: 44}}>🅰️🅰️🅰️®</b>
        <Box mx="auto" />
        <ConnectButton style={{ flexShrink: '0', alignSelf: 'flex-end' }} />
      </Flex>
      <Box
        sx={{
          color: '#000',
          padding: '0',
          margin: '0.625em 0 0 0',
          textAlign: 'right',
          fontSize: [16, 18, 20]
        }}
      >
        {account ? accountInfo : <Box>&nbsp;</Box>}
      </Box>
    </Box>
  )
}