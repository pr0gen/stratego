import math
import random
import time

from multiprocessing import Process, Queue, cpu_count

from ai_python.src.board_generator.position import Position
from ai_python.src.board_generator.board import Board
from ai_python.src.board_generator.evaluation import Eval

def mainV1(number_evaluation, queue):

    eval = Eval()
    board = Board()

    number_boards = 0
    f = open("boards.txt", "a")

    for i in range(number_evaluation):
        board.full_shuffle()

        if eval.can_take_this_board(board.board):
            f.write(board.to_string() + "\n")
            number_boards += 1

    f.close()
    queue.put(number_boards)


def mainV2(number_evaluation, queue):

    eval = Eval()
    board = Board()

    number_boards = 0
    # f = open("boards.txt", "a")

    for i in range(number_evaluation):
        board.full_shuffle()
        score = eval.get_score(board.board)
        if score >= eval.get_max_score():
            # f.write(board.to_string() + "\n")
            number_boards += 1

    # f.close()
    queue.put(number_boards)




def multi_cpu():

    number_total_board = 10000000

    number_cpu = cpu_count()
    number_board_per_cpu = (int) (number_total_board / number_cpu)
    queue = Queue()

    # processes = [Process(target=mainV1, args=(number_board_per_cpu, queue,)) for _ in range(number_cpu)]
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


    print('finished main')


if __name__ == '__main__':

    start_time = time.time()
    multi_cpu()
    # b = Board()
    #
    # for i in range(10):
    #     b.full_shuffle()
    #     b.show()

    # b.show()
    #
    # eval = Eval()
    # score = eval.get_score(b.board)
    # print(score)


    print("--- %s seconds ---" % (time.time() - start_time))


