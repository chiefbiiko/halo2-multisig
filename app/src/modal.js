import { default as ReactModal } from 'react-modal'
import { useSelector, useDispatch } from 'react-redux'
import { Text, Flex, Box } from 'rebass'
import Button from './button'
import { dump } from './redux'

export default function Modal() {
  const dispatch = useDispatch()
  const { modalText, modalTitle } = useSelector(state => state)
  return (
    <ReactModal
      isOpen={!!modalText}
      onRequestClose={() => dispatch(dump({ modalText: null }))}
      contentLabel="info overlay modal"
      appElement={document.getElementById('root')}
      className="responsive-modal"
    >
      <Flex
        style={{
          justifyContent: 'center',
          marginTop: window.innerHeight < 300 ? '0' : '20vh'
        }}
      >
        <Box
          sx={{
            background: '#fff',
            border: '0.1875em solid #000',
            boxShadow: '0 0.625em',
            padding: '0.625em',
            wordWrap: 'break-word',
            textAlign: 'center',
            fontSize: ['1em', '1.25em']
          }}
        >
          <Text sx={{ fontWeight: 'bold', marginBottom: '0.625em' }}>
            {modalTitle}
          </Text>
          <Text sx={{ font: '1em Open Sans, normal', marginBottom: '1.25em' }}>
            {modalText}
          </Text>
          <Button
            aria-label="Ok"
            style={{ width: 'auto', marginBottom: '0.625em' }}
            onClick={() => dispatch(dump({ modalText: null }))}
          >
            Ok
          </Button>
          <div style={{ clear: 'both' }}></div>
        </Box>
      </Flex>
    </ReactModal>
  )
}