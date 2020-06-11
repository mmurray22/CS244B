import numpy as np
import matplotlib.pyplot as plt
from matplotlib import style
import sys
import os

def figure_1a():

    mean_times = [200, 400, 600, 800, 1000, 2000, 3000, 4000]
    results_dir_a = "success_rates/ksize_{}_alpha_{}".format(20, 3)

    success_rates_a = []
    for mean_time in mean_times:
        f = open(results_dir_a + '/meantime_' + str(mean_time), 'r')

        rate = f.read()
        success_rates_a.append(float(rate))

    results_dir_b = "success_rates/ksize_{}_alpha_{}".format(3, 3)

    success_rates_b = []
    for mean_time in mean_times:
        f = open(results_dir_b + '/meantime_' + str(mean_time), 'r')

        rate = f.read()
        success_rates_b.append(float(rate))

    style.use('seaborn')

    fig, ax = plt.subplots()
    ksize_20_alpha_3_plot = ax.plot(np.arange(len(mean_times)), success_rates_a, label = "K Value={}, Alpha={}".format(20, 3), color='blue', marker='^')
    ksize_3_alpha_3_plot = ax.plot(np.arange(len(mean_times)), success_rates_b, label = "K Value={}, Alpha={}".format(3, 3), color='red', marker='s')

    ax.set_xlabel("Mean Time Online (s)")
    ax.set_ylabel("Success Ratio (%)")
    ax.xaxis.set_ticks(np.arange(len(mean_times)))
    ax.xaxis.set_ticklabels(mean_times)
    ax.set_yticks(np.arange(start=0.5, stop=1.1, step=0.1))
    ax.set_xlim([0,8])
    ax.set_ylim([0.0,1.01])
    ax.legend(labelspacing = 1.25, frameon=True)
    plt.savefig("figures/figure_1a")
    plt.show()

def figure_2():

    mean_times = [200, 400, 600, 800, 1000, 2000, 3000, 4000]

    success_rates_1 = []
    success_rates_2 = []
    success_rates_3 = []
    success_rates_4 = []
    success_rates_5 = []
    success_rates = [success_rates_1, success_rates_2, success_rates_3, success_rates_4, success_rates_5]

    for index, success_rate in enumerate(success_rates):

        value = index+1

        results_dir = "success_rates/ksize_{}_alpha_{}".format(value, 1)

        for mean_time in mean_times:
            f = open(results_dir + '/meantime_' + str(mean_time), 'r')

            rate = f.read()
            success_rate.append(float(rate))

    style.use('seaborn')

    fig, ax = plt.subplots()
    ksize_1_alpha_1_plot = ax.plot(np.arange(len(mean_times)), success_rates_1, label = "K Value={}, Alpha={}".format(1, 1), color='black', marker='s')
    ksize_2_alpha_1_plot = ax.plot(np.arange(len(mean_times)), success_rates_2, label = "K Value={}, Alpha={}".format(2, 1), color='blue', marker='^')
    ksize_3_alpha_1_plot = ax.plot(np.arange(len(mean_times)), success_rates_3, label = "K Value={}, Alpha={}".format(3, 1), color='green', marker='o')
    ksize_4_alpha_1_plot = ax.plot(np.arange(len(mean_times)), success_rates_4, label = "K Value={}, Alpha={}".format(4, 1), color='red', marker='s')
    ksize_5_alpha_1_plot = ax.plot(np.arange(len(mean_times)), success_rates_5, label = "K Value={}, Alpha={}".format(5, 1), color='black', ls = ":", marker='^')



    ax.set_xlabel("Mean Time Online (s)")
    ax.set_ylabel("Success Ratio (%)")
    ax.xaxis.set_ticks(np.arange(len(mean_times)))
    ax.xaxis.set_ticklabels(mean_times)
    ax.set_yticks(np.arange(start=0.5, stop=1.1, step=0.1))
    ax.set_xlim([0,8])
    ax.set_ylim([0.0,1.01])
    ax.legend(labelspacing = 1.25, frameon=True)
    plt.savefig("figures/figure_2")
    plt.show()

figure_1a()
figure_2()
