import { delay, put } from 'redux-saga/effects'
import { increment } from './slice'

function* tickTockSaga() {
  while(true) {
    yield delay(1000)
    yield put(increment())
  }
}

export default tickTockSaga
