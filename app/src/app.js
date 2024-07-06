import Home from './home.js'
// import Header from './header.js'
import Footer from './footer.js'
import { Box } from 'rebass'
import Dots from './dots.js'
import Modal from './modal.js'
// import ConnectModal from './connect-modal'
// import SafeModal from './safe-modal'
// import ChainModal from './chain-modal.js'
// import TokenModal from './token-modal.js'
// import RegisterModal from './register-modal.js'
// import RegisterNameModal from './register-name-modal.js'
// import WalletModal from './wallet-modal.js'
// import Menu from './menu'

export default function App() {
  return (
    <>
      <Dots />
      {/* <ConnectModal />
      <SafeModal />
      <ChainModal />
      <TokenModal />
      <WalletModal />
      <RegisterModal />
      <RegisterNameModal /> */}
      <Modal />
      <Box sx={{ height: '100vh', display: 'flex', flexDirection: 'column' }}>
        {/* <Header />
        <Menu /> */}
        <Home />
        <Footer />
      </Box>
    </>
  )
}