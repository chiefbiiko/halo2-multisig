import { useSelector } from 'react-redux'
import { Box, Flex, Text } from 'rebass'

export default function Dots() {
  const { dots, progress } = useSelector(state => state)
  return dots ? (
    <Flex
      sx={{
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        background: 'rgba(255,255,255,.75)',
        position: 'fixed',
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
        zIndex: 419
      }}
    >
      <Box>
        <Text
          sx={{
            margin: '0 0 1.25em 0',
            fontStyle: 'oblique',
            bg: 'rgba(255,255,255,.55)',
            borderRadius: 5,
            padding: '0 0.3125em'
          }}
        >
          {progress}
        </Text>
        <Box
          className="three-dots-pulse"
          sx={{ background: 'transparent', margin: '0 auto' }}
        />
      </Box>
    </Flex>
  ) : null
}