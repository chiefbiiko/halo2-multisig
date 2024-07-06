import React from "react"
import { Flex, Text, Box, Link } from "rebass"

export default function Footer() {
  return (
    <Box bg="#fff">
      <Flex
        sx={{
          color: "#000",
          padding: "0.625em",
          margin: "auto 0 0 0",
          fontSize: [16, 18, 20]
        }}
      >
        <span>🅰️🅰️🅰️®</span>
        <Box mx="auto" />
      </Flex>
    </Box>
  )
}