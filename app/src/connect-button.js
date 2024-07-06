import React from 'react'
import Button from './button'
import { useDispatch, useSelector } from 'react-redux'
import { useMediaQuery } from 'react-responsive'
import { dump } from './redux'
import _MetaMask from '@metamask/sdk'

export default function ConnectButton({ style }) {
//   const { account } = useMyMetaMask()
  const { connected } = useSelector(state => state)
  const dispatch = useDispatch()
  const isSmol = useMediaQuery({ query: '(max-width: 21.875em)' })

  return (
    <Button
      aria-label="Connect wallet"
      sx={{
        width: 'auto',
        ...style
      }}
      onClick={_ => {
        // shieldedAddress ? dispatch(disconnect()) : dispatch(connect(account))
        new _MetaMask({}).getProvider().request({ method: 'eth_requestAccounts' })
        dispatch(dump({connected: true}))
      }}
    >
      {isSmol ? '🔌' : connected ? '🔌' : 'Connect'}
    </Button>
  )
}