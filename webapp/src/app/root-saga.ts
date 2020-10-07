import { fork } from 'redux-saga/effects'
import counterSaga from '../features/counter/saga'

function* rootSaga() {
  yield fork(counterSaga)
}

export default rootSaga
