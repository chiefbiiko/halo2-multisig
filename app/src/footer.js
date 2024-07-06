import React from 'react'
import { Flex, Text, Box, Link } from 'rebass'

export default function Footer() {
  return (
    <Box>
      <Flex
        sx={{
          color: '#000',
          padding: '0.625em',
          margin: 'auto 0 0 0',
          fontSize: [16, 18, 20]
        }}
      >
        <Text>{''}</Text>
        <Box mx="auto" />
        <Link
          href="https://donate.peppersec.com/"
          target="_blank"
          style={{
            color: '#fff',
            textShadow: '0 0 2.5em #000',
            margin: '0.625em 0 0 0.625em',
            textDecoration: 'none',
            fontWeight: 'bold',
            cursor: 'pointer'
          }}
        >
          🅰️🅰️🅰️®
        </Link>
      </Flex>
    </Box>
  )
}