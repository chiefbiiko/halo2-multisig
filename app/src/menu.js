import React from 'react'
import { Flex, Text, Box, Link } from 'rebass'
import { useSelector, useDispatch } from 'react-redux'
import { dump, useMyMetaMask } from './redux'

export default function Menu() {
  const dispatch = useDispatch()
  const { chainName } = useMyMetaMask()
  const { selectedMenu, shieldedAddress, isRegistered, burgerMenuOpen } =
    useSelector(state => state)
  return (
    <Flex
      sx={{
        color: '#000',
        padding: '0 0.625em',
        margin: 0,
        fontSize: [16, 18, 20],
        flexWrap: 'wrap'
      }}
    >
      <Text
        onClick={() =>
          dispatch(
            dump({
              selectedMenu: "sign"
            })
          )
        }
        style={{
          margin: '0 0.625em',
          cursor: "pointer",
          fontWeight: selectedMenu === 'sign' ? 'bold' : 'normal'
        }}
      >
        Sign
      </Text>
      <Text
        onClick={() =>
          dispatch(
            dump({
              selectedMenu: "gen"
            })
          )
        }
        style={{
          margin: '0 0.625em',
          cursor: "pointer",
          fontWeight: selectedMenu === 'gen' ? 'bold' : 'normal'
        }}
      >
        Prove
      </Text>
      <Text
        onClick={() =>
          dispatch(
            dump({
              selectedMenu: "exec"
            })
          )
        }
        style={{
          margin: '0 0.625em',
          cursor: 'pointer',
          fontWeight: selectedMenu === 'exec' ? 'bold' : 'normal'
        }}
      >
        Exec
      </Text>
    </Flex>
  )
}