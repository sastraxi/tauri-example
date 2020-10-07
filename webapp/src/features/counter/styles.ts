import styled from '@emotion/styled'

export const Row = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;

  &:not(:last-child) {
    margin-bottom: 16px;
  } 
`

export const Value = styled.span`
  font-size: 78px;
  padding-left: 16px;
  padding-right: 16px;
  margin-top: 2px;
  font-family: 'Courier New', Courier, monospace;
`

export const Button = styled.button`
  appearance: none;
  background: none;
  font-size: 32px;
  padding-left: 12px;
  padding-right: 12px;
  outline: none;
  border: 2px solid transparent;
  color: rgb(112, 76, 182);
  padding-bottom: 4px;
  cursor: pointer;
  background-color: rgba(112, 76, 182, 0.1);
  border-radius: 2px;
  transition: all 0.15s;
 
  &:hover, &:focus {
    border: 2px solid rgba(112, 76, 182, 0.4);
  }

  &:active {
    background-color: rgba(112, 76, 182, 0.2);
  }
`

export const Textbox = styled.input`
  font-size: 32px;
  padding: 2px;
  width: 64px;
  text-align: center;
  margin-right: 8px;
  background-color: #feffff; /* TODO: report bug in emotion */
`
