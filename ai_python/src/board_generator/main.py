import math
import random
import time

from Board import Board
from Evaluation import Eval
from multiprocessing import Process, Queue, cpu_count

def mainV1(number_evaluation, queue):

    eval = Eval()
    board = Board()

    number_boards = 0
    f = open("boards.txt", "a")

    for i in range(number_evaluation):
        board.full_shuffle()
        score = eval.get_score(board.board)
        if score == eval.get_max_score():
            f.write(board.to_string() + "\n")
            number_boards += 1

    f.close()
    queue.put(number_boards)


def mainV2(number_evaluation, queue):

    eval = Eval()
    score = -math.inf
    save_board = None
    board = Board()

    # for i in range(number_evaluation):
    #     board.full_shuffle()
    #     # board.random_moves(4)
    #     temp_score = eval.get_score(board.board)
    #
    #     if temp_score > score:
    #         score = temp_score
    #         save_board = board.copy()
    #
    # # queue.put((score))
    # queue.put((score, save_board))

    number_boards = 0
    f = open("boards.txt", "a")

    for i in range(number_evaluation):
        board.full_shuffle()
        score = eval.get_score(board.board)

        if score == eval.get_max_score():
            # boards.append(board.copy())
            f.write(board.to_string() + "\n")
            number_boards += 1

    f.close()
    queue.put(number_boards)


def multi_cpu():

    number_total_board = 1000

    number_cpu = cpu_count()
    number_board_per_cpu = (int) (number_total_board / number_cpu)
    queue = Queue()

    processes = [Process(target=mainV1, args=(number_board_per_cpu, queue,)) for _ in range(number_cpu)]

    for processe in processes:
        processe.start()

    for processe in processes:
        processe.join()

    results = [queue.get() for _ in processes]

    total = 0
    for result in results:
        total += result
    print(total)

    best_score = -math.inf
    
    # for result in results:
    #     result[1].show()
    #     print(result[0])
        # if result[0] > best_score:
        #     best_score = result[0]
        #     best_board = result[1]
            
    # print(best_score)
    # print(best_board.show())

    print('finished main')


if __name__ == '__main__':

    start_time = time.time()
    multi_cpu()
    print("--- %s seconds ---" % (time.time() - start_time))


