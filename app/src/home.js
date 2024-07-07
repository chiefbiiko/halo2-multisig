import React from 'react'
import { Box } from 'rebass'
import { useSelector } from 'react-redux'
import Gen from './gen'

export default function Home() {
  const { selectedMenu } = useSelector(state => state)
  let selectedComponent
  // if (selectedMenu === 'sign') {
  //   selectedComponent = <Sign />
  // } 
  // else 
  // if (selectedMenu === "gen") {
  //   selectedComponent = <Gen />
  // } 
  // // else if (selectedMenu === "exec") {
  //   selectedComponent = <Exec />
  // }
  return (
    <Box
      sx={{
        margin: '3vh auto auto auto',
        background: '#fff',
        padding: 0,
        maxWidth: '24em',
        width: 'auto'
      }}
    >
      <Gen />
    </Box>
  )
}