import { Button as _Button } from 'rebass'

const lite = { color: '#000', bg: '#fff' }
const dark = { color: '#fff', bg: '#000' }

export default function Button(props) {
  return (
    <_Button
      theme="lite"
      className="bermuda-button"
      {...{ ...props }}
      sx={{
        border: '0.1875em solid #000',
        boxShadow: '0 0.15625em',
        fontWeight: 'bold',
        fontSize: [16, 18, 20],
        fontFamily: 'Open Sans',
        width: '100%',
        cursor: 'pointer',
        ...(props.theme === 'dark' ? dark : lite),
        ...props.sx,
        ...props.style
      }}
    >
      {props.text || props.children}
    </_Button>
  )
}